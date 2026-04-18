
use crate::{program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}};


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
    TimeFunction { idx:usize, function_type: AggregateType, history: Vec<HistoryValue>, max_bound: Option<usize> },
    AggregateFunction { idx:usize, function_type: AggregateType },
    Foreach { idx:usize }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryValue {
    pub(crate) value: i128, 
    pub(crate) spawn_point: i128
} 

impl From<(i128, i128)> for HistoryValue {
    fn from(value: (i128, i128)) -> Self {
        let (value, spawn_point) = value;
        Self { value, spawn_point }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AggregateType { Sum,  Avg }

#[derive(Debug, PartialEq, Clone)]
pub enum LTL { Always, Eventually(bool) }