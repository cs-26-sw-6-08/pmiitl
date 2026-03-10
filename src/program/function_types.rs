use std::error::Error;

#[derive(Debug, PartialEq)]
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