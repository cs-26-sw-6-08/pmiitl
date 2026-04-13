use std::{collections::HashMap, error::Error, rc::Rc};

use crate::monitor_setup::types::{DerivedOutput, Device, Operation, Verdict};

#[derive(Debug, PartialEq)]
pub struct Streams {
    pub output_streams: Vec<OutputStream>,
    pub devices: Rc<HashMap<i128, Vec<Device>>>,
    pub time_stream: Rc<i128>
}

impl Streams {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Streams {
            output_streams: Vec::new(),
            devices: Rc::new(HashMap::new()),
            time_stream: Rc::new(0)
        })
    }


}

#[derive(Debug, PartialEq)]
pub enum LTL {
    Always, 
    Eventually
}

#[derive(Debug, PartialEq)]
pub struct OutputStream {
    ltl: LTL,
    bound: Option<(i128, i128)>,
    time_verdicts: HashMap<i128, Verdict>,
    operations: Vec<Operation>,
}

impl From<(LTL, Vec<Operation>, Option<(i128, i128)>)> for OutputStream {
    fn from(value: (LTL, Vec<Operation>, Option<(i128, i128)>)) -> Self {
        Self {
            bound: value.2,
            ltl: value.0,
            operations: value.1,
            time_verdicts: HashMap::new()
        }
    }
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

type StreamDerivedSig =  Box<dyn Fn(i128, Option<&Device>, i128) -> DerivedOutput>;
pub struct DerivedStream(
    pub Rc<StreamDerivedSig>
);

impl DerivedStream {
    pub fn from_fn(value: StreamDerivedSig) -> Self {
        Self (Rc::new(value))
    }        

    pub fn clone_rc(&self) -> Rc<StreamDerivedSig> {
        self.0.clone()
    }
}


impl From<StreamDerivedSig> for DerivedStream {    
    fn from(value: StreamDerivedSig) -> Self {
        Self (Rc::new(value))
    }        
}

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
