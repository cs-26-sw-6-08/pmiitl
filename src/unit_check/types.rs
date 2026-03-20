use std::fmt::Display;

use crate::program::units::Unit;

#[derive(Debug, PartialEq)]
pub enum Type {
    Seconds,
    WattSeconds,
    Watt,
    Number,
    String
}

impl Type {
    pub fn unit_type(unit: &Unit) -> Type {
        match unit {
            Unit::Seconds => Type::Seconds,
            Unit::Watt => Type::Watt,
            Unit::WattSeconds => Type::WattSeconds,
            _ => unreachable!()
        }        
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Seconds => write!(f, "Seconds"),
            Type::WattSeconds => write!(f, "WattSeconds"),
            Type::Watt => write!(f, "Watt"),
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
        }
    }
}