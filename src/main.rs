mod grammar;// default namespace for the parser is the grammar's name
pub mod program;
pub mod unit_check;
pub mod equiv_convert;
pub mod unit_convert;
pub mod soundness;
pub mod utils;
pub mod monitor_setup;
pub mod monitor;
pub mod frontend;
mod errors;
extern crate hime_redist;
use colored::Colorize;
use dotenv::dotenv;
use monitor::instrumentation::Instrumentation;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let base_url = std::env::var("BASE_URL").expect("BASE_URL not defined in .env");
    let token = std::env::var("TOKEN").expect("TOKEN not defined in .env");
    let addr = std::env::var("FRONTEND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let program_path = std::env::var("PROGRAM_PATH").unwrap_or_else(|_| "program.txt".into());

    let instrumentation = match Instrumentation::new(&base_url, &token) {
        Ok(instrumentation) => instrumentation,
        Err(err) => return error_print(format!("{}", err)),
    };

    let state = frontend::AppState::new(instrumentation, program_path.into());
    if let Err(err) = frontend::serve(state, &addr).await {
        error_print(format!("{}", err));
    }
}

fn error_print(err: String) {
    println!("{}{}",String::from("[Error]: ").bright_red().bold(), err)
}
