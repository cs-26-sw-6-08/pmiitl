


#[derive(Debug, PartialEq)]
pub enum Verdict { True, False, Undecided }

#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    power: i128,
    active: bool
}

#[derive(Debug, PartialEq)]
pub struct StackValue<'a> {
    value: DerivedOutput<'a>, 
    decided: Decidedability 
}

#[derive(Debug, PartialEq)]
pub enum DerivedOutput<'a> {
    Verdict(Verdict),
    Number(i128),
    String(&'a String)
}


#[derive(Debug, PartialEq)]
pub enum Decidedability { Decided, Undecided }

impl<'a> From<i128> for StackValue<'a> {
    fn from(value: i128) -> Self {
        Self { value: DerivedOutput::Number(value), decided: Decidedability::Decided }
    }
}

impl<'a> From<&'a String> for StackValue<'a> {
    fn from(value: &'a String) -> Self {
        Self { value: DerivedOutput::String(value), decided: Decidedability::Decided }
    }
}

impl<'a> From<Verdict> for StackValue<'a> {
    fn from(value: Verdict) -> Self {
        Self { value: DerivedOutput::Verdict(value), decided: Decidedability::Decided }
    }
}