use crate::program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}};

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
pub enum DerivedOutput<'a> {
    Verdict(Verdict),
    Number(i128),
    String(&'a String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    AlwaysUnbounded { idx: usize },
    AlwaysBounded { bound: (i128,i128), idx: usize },
    Eventually { bound: (i128,i128), idx: usize },
    Binary { bin_op: BinaryOperators, idx_lhs: usize, idx_rhs: usize },
    Unary { un_op: UnaryOperators, idx: usize },
    Number(i128),
    String(String),
    Member(MemberType),
    CurrentTime,
    Sumtime { idx:usize },
    Sum { idx:usize },
    Avg { idx:usize },
    Avgtime { idx:usize },
    Foreach { idx:usize }
}