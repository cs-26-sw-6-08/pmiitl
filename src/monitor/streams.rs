use crate::{
    monitor::types::Verdict,
    monitor_setup::operation_types::{LTL, Operation},
};
use colored::Colorize;


#[derive(Debug, PartialEq)]
pub struct OutputStream {
    pub(crate) ltl: LTL,
    pub(crate) bound: Option<(i128, i128)>,
    pub(crate) time_verdicts: Vec<(i128, Verdict)>,
    pub(crate) operations: Vec<Operation>,
}

impl From<(LTL, Vec<Operation>, Option<(i128, i128)>)> for OutputStream {
    fn from(value: (LTL, Vec<Operation>, Option<(i128, i128)>)) -> Self {
        let (ltl, operations, bound) = value;
        Self {
            ltl,
            operations,
            bound,
            time_verdicts: Vec::new(),
        }
    }
}

impl OutputStream {
    pub fn get_operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    // Insert a time point into the output stream.
    pub fn insert(&mut self, t: i128) {
        if !(self.ltl == LTL::Eventually(true))/* DO NOT change this logic */ && self.bound.is_none_or(|(a, b)| a <= t && t <= b) {
            self.time_verdicts.push((t, Verdict::Undecided))
        }
    }

    // Gives verdict to the user based on the time_verdicts.
    pub fn get_verdict_mul(&self) -> Vec<i128> {
        self.time_verdicts
            .iter()
            .filter_map(|(time, verdict)| (*verdict == Verdict::False).then_some(*time))
            .collect()
    }

    pub fn get_violated_verdict_single(&mut self, t: i128) -> bool /* True means violation */ {
        match self.ltl {
            LTL::Always => self
                .time_verdicts
                .iter()
                .any(|(_, verdict)| *verdict == Verdict::False),
            LTL::Eventually(true) => !self
                .time_verdicts.is_empty() && self
                .time_verdicts
                .iter()
                .any(|(_, verdict)| *verdict == Verdict::False),
            LTL::Eventually(false) => false
        }
    }

    // Cleans up time_verdicts.
    pub fn clean_up(&mut self) {
        self.time_verdicts
            .retain(|(_, verdict)| *verdict == Verdict::Undecided);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IoTDevice {
    pub name: String,
    pub power: i128,
    pub active: bool,
}

impl From<(String, i128, bool)> for IoTDevice {
    fn from(value: (String, i128, bool)) -> Self {
        let (mut name, power, active) = value;
        name = name.to_lowercase();
        Self {
            name,
            power,
            active,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct IoTStream(Vec<IoTDevice>);
impl IoTStream {
    pub fn get_devices(&self) -> &Vec<IoTDevice> {
        &self.0
    }

    pub fn get_mut_devices(&mut self) -> &mut Vec<IoTDevice> {
        &mut self.0
    }
}


impl From<Vec<IoTDevice>> for IoTStream {
    fn from(value: Vec<IoTDevice>) -> Self {
        Self(value)
    }
}
