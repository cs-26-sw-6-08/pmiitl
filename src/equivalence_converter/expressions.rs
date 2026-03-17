use std::error::Error;

use crate::{equivalence_converter::conversion_binary_operator::conversion_binary_operations, program::{expressions::Expr, function_types::FunctionType}};

impl Expr {
    pub fn convert(&self) -> Result<Expr, Box<dyn Error>> {
        match self {
            Expr::BinaryOperations { lhs, rhs, operator } => {
                let lhs = lhs.convert()?;
                let rhs = rhs.convert()?;

                Ok(conversion_binary_operations(lhs, rhs, operator)?)
            },
            Expr::Always { interval, not, expr } => {
                Ok(Expr::Always { interval: interval.clone().and_then(|e| Some(e.convert().ok()?.into())), not: *not, expr: expr.convert()?.into() })
            },
            Expr::Unit { number, unit: _ } => {
                Ok(number.convert()?)
            }
            Expr::Function { aggregate_type, expr } => match aggregate_type {
                FunctionType::Count => {
                    let expr = expr.convert()?;
                    Ok(Expr::Function { aggregate_type: FunctionType::Sum, expr: expr.into() })
                },
                _ => Ok(self.clone())
            }

            _ => Ok(self.clone()),
        }
    }
}