use std::error::Error;

#[derive(Debug)]
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