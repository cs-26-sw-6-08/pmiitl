use std::error::Error;

use crate::{
    equiv_convert::conversion_binary_operator::conversion_binary_operations,
    program::{expressions::Expr, function_types::FunctionType},
};

impl Expr {
    //minimize tree by calculating simple arithmetic and boolean expressions and making certain equivalence conversions
    pub fn equiv_convert(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Expr::BinaryOperations { lhs, rhs, operator } => {
                lhs.equiv_convert()?;
                rhs.equiv_convert()?;

                *self = conversion_binary_operations(*lhs.clone(), *rhs.clone(), operator.clone())?;
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
                    *expr = match expr.as_ref() {
                        Expr::Number(n) => Expr::Boolean(*n != 0).into(),
                        Expr::Boolean(n) => Expr::Boolean(*n).into(),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                FunctionType::Counttime => {
                    expr.equiv_convert()?;

                    *aggregate_type = FunctionType::Sumtime;

                    *expr = match expr.as_ref() {
                            Expr::Number(n) => Expr::Boolean(*n != 0).into(),
                            Expr::Boolean(n) => Expr::Boolean(*n).into(),
                            _ => unreachable!(),
                        };
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
