
use crate::{program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}};


#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    LTLAlwaysUnbounded { idx: usize },
    LTLBounded { bound: (i128,i128), idx: usize, not: bool, ltl_type: ExprLTL },
    Binary { bin_op: BinaryOperators, idx_lhs: usize, idx_rhs: usize },
    Unary { un_op: UnaryOperators, idx: usize },
    Number(i128),
    String(String),
    Member(MemberType),
    SpawnTime,
    TimeFunction { idx:usize, function_type: AggregateType, history: Vec<HistoryValue<i128>>, bound: i128 },
    AggregateFunction { idx:usize, function_type: AggregateType },
    Foreach { idx:usize }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryValue<T> {
    pub(crate) value: T, 
    pub(crate) spawn_point: i128
} 

impl From<(i128, i128)> for HistoryValue<i128> {
    fn from(value: (i128, i128)) -> Self {
        let (value, spawn_point) = value;
        Self { value, spawn_point }
    }
}

impl From<(bool, i128)> for HistoryValue<bool> {
    fn from(value: (bool, i128)) -> Self {
        let (value, spawn_point) = value;
        Self { value, spawn_point }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AggregateType { Sum, Avg }

#[derive(Debug, PartialEq, Clone)]
pub enum PropLTL { Always, Eventually(bool) }
#[derive(Debug, PartialEq, Clone)]
pub enum ExprLTL { Always, Eventually(Vec<HistoryValue<bool>>) }