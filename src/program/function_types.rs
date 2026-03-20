use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionType {
    Sum,
    Avg,
    Count,
    Sumtime,
    Avgtime,
    Counttime,
    Foreach
}

impl FunctionType {
    pub fn new(operator: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match operator {
            "sum" => Self::Sum,
            "avg" => Self::Avg, 
            "count" => Self::Count,
            "sumtime" => Self::Sumtime,
            "avgtime" => Self::Avgtime,
            "counttime" => Self::Counttime,
            "foreach" => Self::Foreach,
            _ => unreachable!()
        })
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionType::Sum => write!(f, "Sum"),
            FunctionType::Avg => write!(f, "Avg"),
            FunctionType::Count => write!(f, "Count"),
            FunctionType::Sumtime => write!(f, "Sumtime"),
            FunctionType::Avgtime => write!(f, "Avgtime"),
            FunctionType::Counttime => write!(f, "Counttime"),
            FunctionType::Foreach => write!(f, "Foreach"),
        }
    }
}