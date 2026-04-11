use std::{collections::HashMap, error::Error, rc::Rc, sync::Arc};

use crate::monitor_setup::types::{DerivedOutput, Device, Verdict};

#[derive(Debug, PartialEq)]
pub struct Streams {
    pub output_streams: Vec<OutputStream>,
    pub devices: Arc<HashMap<i128, Vec<Device>>>,
    pub time_stream: Arc<i128>
}

impl Streams {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Streams {
            output_streams: Vec::new(),
            devices: Arc::new(HashMap::new()),
            time_stream: Arc::new(0)
        })
    }


}

#[derive(Debug, PartialEq)]
pub struct OutputStream {
    bound: Option<(i128, i128)>,
    time_verdicts: HashMap<i128, Verdict>,
    derived_streams: Vec<DerivedStream>,
}

impl OutputStream {
    // Insert a time point into the output stream.
    fn insert(&mut self, t: i128) {
        if let Some((a, b)) = self.bound {
            if a <= t && t <= b {
                self.time_verdicts.insert(t, Verdict::Undecided);
            }
        } else {
            self.time_verdicts.insert(t, Verdict::Undecided);
        }
    }

    // Calculate the verdict for the output stream.
    fn update(&mut self) {
        todo!()
    }

    // Gives verdict to the user based on the time_verdicts.
    fn get_verdict_mul(&self) -> Vec<i128> {
        self.time_verdicts
            .iter()
            .filter_map(|(&time, verdict)| {
                if *verdict == Verdict::False {
                    Some(time)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_verdict_single(&self) -> bool {
        self.time_verdicts
            .iter()
            .any(|(_, verdict)| *verdict == Verdict::False)
    }

    // Cleans up time_verdicts.
    fn clean_up(&mut self) {
        self.time_verdicts
            .retain(|_, verdict| *verdict == Verdict::Undecided);
    }
}

type StreamDerivedSig =  Box<dyn for<'a> Fn(i128, Option<&'a Device>, i128) -> DerivedOutput<'a>>;
pub struct DerivedStream(
    pub Rc<StreamDerivedSig>
);

impl DerivedStream {
    pub fn from_fn(value: StreamDerivedSig) -> Self {
        Self (Rc::new(value))
    }        

    pub fn clone_arc(&self) -> Rc<StreamDerivedSig> {
        self.0.clone()
    }
}

/* 
impl From<Box<dyn Fn(i128, Option<Device>, i128) -> DerivedOutput + 'static>> for DerivedStream {    
    fn from(value: Box<dyn Fn(i128, Option<Device>, i128) -> DerivedOutput + 'static>) -> Self {
        Self (value)
    }        
}*/

impl std::fmt::Debug for DerivedStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DerivedStream").field(&"<closure>").finish()
    }
}

impl PartialEq for DerivedStream {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0.as_ref(), other.0.as_ref())
    }
}
