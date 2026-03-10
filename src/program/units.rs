use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
    Watt,
    KiloWattHours,
}

impl Unit {
    pub fn new(unit: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match unit {
            "s" => Self::Seconds,
            "m" => Self::Minutes,
            "h" => Self::Hours,
            "w" => Self::Watt,
            "kwh" => Self::KiloWattHours,
            _ => unreachable!()
        })
    }
}
