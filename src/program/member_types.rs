use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum MemberType {
    Active,
    Power,
    Name,
}

impl MemberType {
    pub fn new(operator: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match operator {
            "active" => Self::Active,
            "power" => Self::Power,
            "name" => Self::Name,
            _ => unreachable!()
        })
    }
}

impl Display for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberType::Active => write!(f, "Active"),
            MemberType::Power => write!(f, "Power"),
            MemberType::Name => write!(f, "Name"),
        }
    }
}