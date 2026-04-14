
use std::error::Error;
use crate::{
    errors,
    monitor_setup::operation_types::{LTL, Operation, AggregateType}, 
    program::{expressions::Expr,function_types::FunctionType}, utils::vec_helper_funcs::ExtVec
};
use crate::program::operations::BinaryOperators::*;

impl Expr {
    pub fn compile_expression(&self) -> Result<Vec<Operation>, Box<dyn Error>> {
        self
        .compile_expression_helper(Vec::new(), 0)
        .map(|res| res.0)
    }

    fn compile_expression_helper(
        &self,
        streams: Vec<Operation>, 
        key: usize,
    ) -> Result<(Vec<Operation>, usize), Box<dyn Error>> {   
        Ok(match self {
            Expr::Number(c) => (streams.with(Operation::Number(*c)), key+1),
            Expr::String(str) => (streams.with(Operation::String(str.to_owned())),key+1),
            Expr::CurrentTime => (streams.with(Operation::CurrentTime),key+1),
            Expr::Member { access_type } => (streams.with(Operation::Member(access_type.clone())), key+1),
            Expr::Always { interval: None, not: false, expr } => {
                let (new_streams, new_key) = expr.compile_expression_helper(Vec::new(), key+1)?;
                (streams.with(Operation::LTLAlwaysUnbounded { idx: key+1 }).chain(new_streams), new_key)
            },
            Expr::Always { interval: Some(val), not, expr } |
            Expr::Eventually { interval: Some(val), not, expr } => {
                let (new_streams, new_key) = expr.compile_expression_helper(Vec::new(), key+1)?;
                (streams.with(Operation::LTLBounded { 
                    bound: val.get_bound()?, 
                    idx: key+1, 
                    not: *not, 
                    ltl_type: match self {
                        Expr::Always { .. } => LTL::Always,
                        _ => LTL::Eventually
                    }
                }).chain(new_streams), new_key)
            },
            Expr::BinaryOperations { lhs, rhs, operator } => {
                let (new_1_streams, new_1_key) = lhs.compile_expression_helper(Vec::new(), key+1)?;
                let (new_2_streams, new_2_key) = rhs.compile_expression_helper(Vec::new(), new_1_key)?;
                
                (streams.with(Operation::Binary { 
                    bin_op: match operator {
                        And | Implies => Err(errors::Error::InvalidCompileExpr),
                        val => Ok(val.clone())
                    }?, 
                    idx_lhs: key+1, 
                    idx_rhs: new_1_key 
                }).chain(new_1_streams).chain(new_2_streams), new_2_key)
            },
            Expr::UnaryOperations { operand, operator } => {
                let (new_streams, new_key) = operand.compile_expression_helper(Vec::new(), key+1)?;
                (streams.with(Operation::Unary { un_op: operator.clone(), idx: key+1 }).chain(new_streams), new_key)
            },
            Expr::Function { aggregate_type, expr } => match aggregate_type {
                FunctionType::Foreach => {
                    let (new_streams, new_key) = expr.compile_expression_helper(Vec::new(), key + 1)?;
                    (streams.with(
                        Operation::Foreach { idx: key + 1 }
                    ).chain(new_streams), new_key)
                }
                FunctionType::Sum |
                FunctionType::Avg => {
                    let (new_streams, new_key) = expr.compile_expression_helper(Vec::new(), key + 1)?;
                    (streams.with(
                        Operation::AggregateFunction { 
                            function_type: match aggregate_type {
                                FunctionType::Sum => AggregateType::Sum,
                                _ => AggregateType::Avg,
                            }, 
                            idx: key + 1
                        }
                    ).chain(new_streams), new_key)
                },
                FunctionType::Sumtime | FunctionType::Avgtime => {
                    let wrap_function = Expr::Function { aggregate_type: FunctionType::Sum, expr: expr.clone() };
                    let (new_streams, new_key) = wrap_function.compile_expression_helper(Vec::new(), key + 1)?;
                    (
                        streams.with( Operation::TimeFunction { 
                            function_type:  match aggregate_type {
                                FunctionType::Sumtime => AggregateType::Sum,
                                _ => AggregateType::Avg
                            }, 
                            history: Vec::new(), 
                            idx: key + 1 
                        }).chain(new_streams), 
                        new_key
                    )
                },
                _ => Err(errors::Error::InvalidCompileExpr)?,
            },
            Expr::Always { interval: None, not: true, expr: _ } |
            Expr::Interval { .. } |
            Expr::Unit { .. } |
            Expr::Eventually { interval: None, .. } => Err(errors::Error::InvalidCompileExpr)?,
        })
    }
}

