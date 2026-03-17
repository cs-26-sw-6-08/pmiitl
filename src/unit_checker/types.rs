use crate::program::units::Unit;

#[derive(Debug, PartialEq)]
pub enum Type {
    Seconds,
    WattSeconds,
    Watt,
    Number,
    String,
    Bool
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