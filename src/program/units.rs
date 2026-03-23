use std::{error::Error, fmt::Display};

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

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Seconds => write!(f, "Seconds"),
            Unit::Minutes => write!(f, "Minutes"),
            Unit::Hours => write!(f, "Hours"),
            Unit::Watt => write!(f, "Watt"),
            Unit::KiloWatts => write!(f, "KiloWatts"),
            Unit::KiloWattHours => write!(f, "KiloWattHours"),
            Unit::WattHours => write!(f, "WattHours"),
            Unit::WattMinutes => write!(f, "WattMinutes"),
            Unit::WattSeconds => write!(f, "WattSeconds"),
        }
    }
}