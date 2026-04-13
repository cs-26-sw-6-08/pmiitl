use std::collections::HashMap;

use crate::monitor_setup::types::{Operation, Verdict};

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
            ltl: value.0,
            operations: value.1,
            bound: value.2,
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
