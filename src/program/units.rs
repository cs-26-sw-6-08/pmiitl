use std::error::Error;

#[derive(Debug, PartialEq)]
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
            "W" => Self::Watt,
            "kWh" => Self::KiloWattHours,
            _ => unreachable!()
        })
    }
}
