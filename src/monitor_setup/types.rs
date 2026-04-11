use std::rc::Rc;


#[derive(Debug, PartialEq)]
pub enum Verdict {
    True,
    False, 
    Undecided
}


#[derive(Debug, PartialEq)]
pub struct Device {
    name: String,
    power: i128,
    active: bool
}



#[derive(Debug, PartialEq)]
pub enum DerivedOutput {
    Verdict(Verdict),
    Number(i128),
    String(Rc<str>)
}

