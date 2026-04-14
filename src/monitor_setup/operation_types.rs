
use crate::program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}};

#[derive(Debug, PartialEq, Clone)]
pub enum LTL {
    Always, 
    Eventually
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    LTLAlwaysUnbounded { idx: usize },
    LTLBounded { bound: (i128,i128), idx: usize, not: bool, ltl_type: LTL },
    Binary { bin_op: BinaryOperators, idx_lhs: usize, idx_rhs: usize },
    Unary { un_op: UnaryOperators, idx: usize },
    Number(i128),
    String(String),
    Member(MemberType),
    CurrentTime,
    TimeFunction { idx:usize, function_type: AggregateType, history: Vec<i128> },
    AggregateFunction { idx:usize, function_type: AggregateType },
    Foreach { idx:usize }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AggregateType { Sum,  Avg}