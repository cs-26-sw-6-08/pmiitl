use std::error::Error;

use crate::{
    equiv_convert::binary_operations::binary_operations, errors, program::{expressions::Expr, function_types::FunctionType, operations::UnaryOperators}
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
                not,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.equiv_convert()?;
                }
                expr.equiv_convert()?;
                // !always p is equivalent to eventually !p
                if *not {
                    if let Expr::UnaryOperations{operand, operator: UnaryOperators::Not} = expr.as_ref() {
                        *expr = operand.clone();
                    } else {
                        *expr = Expr::UnaryOperations { operand: expr.clone(), operator: UnaryOperators::Not }.into();
                    }
                    *self = Expr::Eventually { interval: interval.clone(), not: false, expr: expr.clone() };
                }
                Ok(())
            },
            Expr::Eventually {
                interval,
                not,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.equiv_convert()?;
                }
                expr.equiv_convert()?;
                // !eventually p is equivalent to always !p
                if *not {
                    if let Expr::UnaryOperations{operand, operator: UnaryOperators::Not} = expr.as_ref() {
                        *expr = operand.clone();
                    } else {
                        *expr = Expr::UnaryOperations { operand: expr.clone(), operator: UnaryOperators::Not }.into();
                    }
                    *self = Expr::Always { interval: interval.clone(), not: false, expr: expr.clone() };
                }
                Ok(())

                
            },
            Expr::Unit { number, unit: _ } => {
                number.equiv_convert()?;
                *self = number.as_ref().clone();
                Ok(())
            },
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
                _ => {
                    expr.equiv_convert()?;
                    Ok(())
                },
            },
            Expr::Interval { start, end } => {
                start.equiv_convert()?;
                end.equiv_convert()?;
                let Expr::Number(start_value) = start.as_ref() else { unreachable!() };
                let Expr::Number(end_value) = end.as_ref() else { unreachable!() };
                if *start_value < 0 || *end_value < 0 {
                    return Err(errors::Error::IntervalBelowZero(*start_value, *end_value).into());
                }
                if *start_value > *end_value {
                    return Err(errors::Error::IntervalStartGreaterThanEnd(*start_value, *end_value).into());
                }
                Ok(())
                
            },
            Expr::UnaryOperations {
                operand,
                operator: _,
            } => {
                operand.equiv_convert()?; 
                Ok(())
            },
            _ => Ok(()),
        }
    }
}
