


#[derive(Debug, PartialEq)]
pub enum Verdict { True, False, Undecided }

#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    power: i128,
    active: bool
}

#[derive(Debug, PartialEq)]
pub enum DerivedOutput<'a> {
    Verdict(Verdict),
    Number(Number),
    String(&'a String)
}

#[derive(Debug, PartialEq)]
pub struct Number {
    num: i128, 
    taint: Taint 
}

#[derive(Debug, PartialEq)]
pub enum Taint { Safe, Tainted }

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Self { num: value, taint: Taint::Safe }
    }
}