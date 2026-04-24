use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum MemberType {
    Power,
    Name,
}

impl MemberType {
    pub fn new(operator: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match operator {
            "power" => Self::Power,
            "name" => Self::Name,
            _ => unreachable!()
        })
    }
}

impl Display for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberType::Power => write!(f, "Power"),
            MemberType::Name => write!(f, "Name"),
        }
    }
}