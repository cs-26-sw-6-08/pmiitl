//! Web frontend: serves a single page where properties can be edited, pushed to the
//! monitor (which restarts it) and where compile errors and violations are shown.

use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::{monitor::instrumentation::Instrumentation, program::Program};

const INDEX_HTML: &str = include_str!("index.html");
const MONITOR_INTERVAL_MS: i128 = 1_000;
/// Oldest output lines are dropped beyond this, so a long-running monitor can't grow memory unbounded
const MAX_LINES: usize = 5_000;

/// Runs the full compilation pipeline on a properties source text.
pub fn compile(source: &str) -> Result<Program, Box<dyn Error>> {
    let mut program = Program::new(source)?;
    program.unit_convert()?;
    program.unit_check()?;
    program.equiv_convert()?;
    program.soundness_check()?;
    program.compile_properties()?;
    program.compute_iotstream_len()?;
    Ok(program)
}

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum LineKind {
    Info,
    Violation,
    /// A timepoint where no property was violated
    Ok,
    Error,
}

#[derive(Serialize, Clone)]
struct Line {
    kind: LineKind,
    text: String,
}

struct Inner {
    properties: String,
    /// Incremented every time the monitor is restarted; the client clears its output when it changes
    generation: u64,
    /// Total number of lines ever pushed in this generation (lines[0] has index `total - lines.len()`)
    total: usize,
    lines: Vec<Line>,
    task: Option<JoinHandle<()>>,
}

impl Inner {
    fn push(&mut self, kind: LineKind, text: String) {
        self.lines.push(Line { kind, text });
        self.total += 1;
        if self.lines.len() > MAX_LINES {
            let excess = self.lines.len() - MAX_LINES;
            self.lines.drain(..excess);
        }
    }
}

pub struct AppState {
    instrumentation: Instrumentation,
    program_path: PathBuf,
    inner: Mutex<Inner>,
}

type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(instrumentation: Instrumentation, program_path: PathBuf) -> SharedState {
        let properties = std::fs::read_to_string(&program_path).unwrap_or_default();
        Arc::new(AppState {
            instrumentation,
            program_path,
            inner: Mutex::new(Inner {
                properties,
                generation: 0,
                total: 0,
                lines: Vec::new(),
                task: None,
            }),
        })
    }
}

/// Compiles `source` and, if it succeeds, stops the running monitor and starts a new one.
/// On a compile error the running monitor is left untouched and the error is returned.
fn apply_properties(state: &SharedState, source: String, save: bool) -> Result<(), String> {
    let program = compile(&source).map_err(|e| e.to_string());

    let mut inner = state.inner.lock().unwrap();
    inner.properties = source.clone();

    let program = match program {
        Ok(program) => program,
        Err(err) => {
            inner.push(LineKind::Error, format!("Compile error: {err}"));
            println!("[Error]: {err}");
            return Err(err);
        }
    };

    if let Some(task) = inner.task.take() {
        task.abort();
    }
    inner.generation += 1;
    inner.total = 0;
    inner.lines.clear();

    if save && let Err(err) = std::fs::write(&state.program_path, &source) {
        inner.push(
            LineKind::Error,
            format!(
                "Could not save properties to {}: {err}",
                state.program_path.display()
            ),
        );
    }

    // Source line of each property, to make violation messages point back into the memo
    let prop_lines: Vec<usize> = program.expressions.iter().map(|e| e.line).collect();
    inner.push(
        LineKind::Info,
        format!("Monitor started with {} properties", prop_lines.len()),
    );
    println!("Monitor (re)started with {} properties", prop_lines.len());

    let generation = inner.generation;
    let task_state = state.clone();
    let instrumentation = state.instrumentation.clone();
    inner.task = Some(tokio::spawn(async move {
        let mut program = program;
        let push = |kind: LineKind, text: String| {
            let mut inner = task_state.inner.lock().unwrap();
            // Ignore output from a monitor that has been replaced
            if inner.generation == generation {
                inner.push(kind, text);
            }
        };

        let result = program
            .monitor(
                instrumentation,
                MONITOR_INTERVAL_MS,
                false,
                |prop_num, t| {
                    let text = match prop_lines.get(prop_num) {
                        Some(line) => {
                            format!("Prop {} (line {line}) violated at time: {t}s", prop_num + 1)
                        }
                        None => format!("Prop {} violated at time: {t}s", prop_num + 1),
                    };
                    println!("\t{text}");
                    push(LineKind::Violation, text);
                },
                |t| push(LineKind::Ok, format!("No violation at time: {t}s")),
            )
            .await
            .map_err(|e| e.to_string());

        if let Err(err) = result {
            println!("[Error]: {err}");
            push(LineKind::Error, format!("Monitor stopped: {err}"));
        }
    }));

    Ok(())
}

#[derive(Deserialize)]
struct PushRequest {
    properties: String,
}

#[derive(Serialize)]
struct PushResponse {
    ok: bool,
    error: Option<String>,
}

async fn push_properties(
    State(state): State<SharedState>,
    Json(req): Json<PushRequest>,
) -> Json<PushResponse> {
    match apply_properties(&state, req.properties, true) {
        Ok(()) => Json(PushResponse {
            ok: true,
            error: None,
        }),
        Err(err) => Json(PushResponse {
            ok: false,
            error: Some(err),
        }),
    }
}

#[derive(Serialize)]
struct PropertiesResponse {
    properties: String,
}

async fn get_properties(State(state): State<SharedState>) -> Json<PropertiesResponse> {
    let inner = state.inner.lock().unwrap();
    Json(PropertiesResponse {
        properties: inner.properties.clone(),
    })
}

#[derive(Deserialize)]
struct OutputQuery {
    generation: Option<u64>,
    since: Option<usize>,
}

#[derive(Serialize)]
struct OutputResponse {
    generation: u64,
    /// Index to pass as `since` on the next poll
    next: usize,
    lines: Vec<Line>,
}

async fn get_output(
    State(state): State<SharedState>,
    Query(q): Query<OutputQuery>,
) -> Json<OutputResponse> {
    let inner = state.inner.lock().unwrap();
    // A new generation means the client must start over from the beginning
    let since = if q.generation == Some(inner.generation) {
        q.since.unwrap_or(0)
    } else {
        0
    };
    let first = inner.total - inner.lines.len();
    let start = since.saturating_sub(first).min(inner.lines.len());
    Json(OutputResponse {
        generation: inner.generation,
        next: inner.total,
        lines: inner.lines[start..].to_vec(),
    })
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Starts the monitor with the saved properties (if they compile) and serves the frontend.
pub async fn serve(state: SharedState, addr: &str) -> Result<(), Box<dyn Error>> {
    let initial = state.inner.lock().unwrap().properties.clone();
    if !initial.trim().is_empty() {
        let _ = apply_properties(&state, initial, false);
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/api/properties", get(get_properties).post(push_properties))
        .route("/api/output", get(get_output))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Frontend available at http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
