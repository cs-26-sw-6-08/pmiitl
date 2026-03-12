use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
    Watt,
    KiloWatts,
    KiloWattHours,
    WattHours,
    WattMinutes,
    WattSeconds,
}

impl Unit {
    pub fn new(unit: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match unit {
            "s" => Self::Seconds,
            "m" => Self::Minutes,
            "h" => Self::Hours,
            "w" => Self::Watt,
            "kw" => Self::KiloWatts,
            "kwh" => Self::KiloWattHours,
            "wh" => Self::WattHours,
            "wm" => Self::WattMinutes,
            "ws" => Self::WattSeconds,
            _ => unreachable!()
        })
    }
}