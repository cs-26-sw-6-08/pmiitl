use std::{collections::HashMap, rc::Rc};

use crate::{monitor_setup::{streams::DerivedStream, types::{DerivedOutput, Device, Operation}}, program::{expressions::Expr,function_types::FunctionType}};


impl Expr {
    pub fn compile_expression(&self) -> Vec<Operation> {
        self.compile_expression_helper(Vec::new(), 0).0
    }

    fn compile_expression_helper(
        &self,
        mut streams: Vec<Operation>, 
        key: usize,
    ) -> (Vec<Operation>, usize) {   
        match self {
            Expr::Number(c) => {
                todo!()
            },
            Expr::String(str) => {
                todo!()
            },
            Expr::CurrentTime => {
                todo!()
            },
            Expr::Unit { number, unit } => unreachable!(),
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
                FunctionType::Count|FunctionType::Counttime => unreachable!(),
            },
        }
    }
}

