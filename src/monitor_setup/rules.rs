
use std::error::Error;
use crate::{
    errors,
    monitor_setup::types::Operation, 
    program::{expressions::Expr,function_types::FunctionType}, utils::vec_helper_funcs::ExtVec
};


impl Expr {
    pub fn compile_expression(&self) -> Result<Vec<Operation>, Box<dyn Error>> {
        self
        .compile_expression_helper(Vec::new(), 0)
        .map(|res| res.0)
    }

    fn compile_expression_helper(
        &self,
        mut streams: Vec<Operation>, 
        key: usize,
    ) -> Result<(Vec<Operation>, usize), Box<dyn Error>> {   
        match self {
            Expr::Number(c) => Ok((streams.with(Operation::Number(*c)), key)),
            Expr::String(str) => {
                todo!()
            },
            Expr::CurrentTime => {
                todo!()
            },
            Expr::Unit { number, unit } => Err(errors::Error::InvalidCompileExpr.into()),
            Expr::Interval { start, end } => todo!(),
            Expr::Always { interval, not, expr } => todo!(),
            Expr::Eventually { interval, not, expr } => todo!(),
            Expr::BinaryOperations { lhs, rhs, operator } => todo!(),
            Expr::UnaryOperations { operand, operator } => todo!(),
            Expr::Member { access_type } => todo!(),
            Expr::Function { aggregate_type, expr } => match aggregate_type {
                FunctionType::Sum => {
                    todo!()
                },
                FunctionType::Avg => todo!(),
                FunctionType::Sumtime => todo!(),
                FunctionType::Avgtime => todo!(),
                FunctionType::Foreach => todo!(),
                FunctionType::Count|FunctionType::Counttime => Err(errors::Error::InvalidCompileExpr.into()),
            },
        }
    }
}

