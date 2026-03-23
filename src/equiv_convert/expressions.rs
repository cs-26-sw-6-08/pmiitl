use std::error::Error;

use crate::{
    equiv_convert::binary_operations::binary_operations,
    program::{expressions::Expr, function_types::FunctionType},
};

impl Expr {
    //minimize tree by calculating simple arithmetic and boolean expressions and making certain equivalence conversions
    pub fn equiv_convert(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Expr::BinaryOperations { lhs, rhs, operator } => {
                lhs.equiv_convert()?;
                rhs.equiv_convert()?;

                *self = binary_operations(*lhs.clone(), *rhs.clone(), operator.clone())?;
                Ok(())
            }
            Expr::Always {
                interval,
                not: _,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.equiv_convert()?;
                }
                expr.equiv_convert()?;
                Ok(())
            }
            Expr::Unit { number, unit: _ } => Ok(number.equiv_convert()?),
            Expr::Function {
                aggregate_type,
                expr,
            } => match aggregate_type {
                FunctionType::Count => {
                    expr.equiv_convert()?;

                    *aggregate_type = FunctionType::Sum;
                    Ok(())
                }
                FunctionType::Counttime => {
                    expr.equiv_convert()?;

                    *aggregate_type = FunctionType::Sumtime;
                    Ok(())
                    
                }
                _ => Ok(expr.equiv_convert()?),
            },
            Expr::Interval { start, end } => {
                start.equiv_convert()?;
                end.equiv_convert()?;
                Ok(())
            },
            Expr::Eventually {
                interval,
                not: _,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.equiv_convert()?;
                }
                expr.equiv_convert()?;
                Ok(())
            }
            Expr::UnaryOperations {
                operand,
                operator: _,
            } => Ok(operand.equiv_convert()?),
            _ => Ok(()),
        }
    }
}
