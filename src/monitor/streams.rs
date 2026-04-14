use crate::{monitor::types::Verdict, monitor_setup::operation_types::{LTL, Operation}};


#[derive(Debug, PartialEq)]
pub struct OutputStream {
    ltl: LTL,
    bound: Option<(i128, i128)>,
    time_verdicts: Vec<(i128, Verdict)>,
    operations: Vec<Operation>,
}

impl From<(LTL, Vec<Operation>, Option<(i128, i128)>)> for OutputStream {
    fn from(value: (LTL, Vec<Operation>, Option<(i128, i128)>)) -> Self {
        let (ltl, operations, bound) = value;
        Self { ltl, operations, bound, time_verdicts: Vec::new() }
    }
}

impl OutputStream {
    pub fn get_operations(&self) -> &Vec<Operation> { &self.operations }

    // Insert a time point into the output stream.
    pub fn insert(&mut self, t: i128) {
        if self.bound.is_none_or(|(a,b)| a <= t && t <= b) {
            self.time_verdicts.push((t, Verdict::Undecided))
        }
    }

    // Calculate the verdict for the output stream.
    pub fn update(&mut self) {
        todo!()
    }

    // Gives verdict to the user based on the time_verdicts.
    pub fn get_verdict_mul(&self) -> Vec<i128> {
        self.time_verdicts
        .iter()
        .filter_map(|(time, verdict)| (*verdict == Verdict::False).then_some(*time))
        .collect()
    }


    pub fn get_violated_verdict_single(&self) -> bool {
        self.time_verdicts
            .iter()
            .any(|(_, verdict)| *verdict == Verdict::False)
    }

    // Cleans up time_verdicts.
    pub fn clean_up(&mut self) {
        self.time_verdicts.retain(|(_, verdict)| *verdict == Verdict::Undecided);
    }
}
