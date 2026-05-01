use std::error::Error;

use crate::{
    equiv_convert::binary_operations::binary_operations,
    errors,
    program::{expressions::Expr, function_types::FunctionType, operations::{BinaryOperators, UnaryOperators}},
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

                // !always p is equivalent to eventually !p
                if *not {
                    if let Expr::UnaryOperations {
                        operand,
                        operator: UnaryOperators::Not,
                    } = expr.as_ref()
                    {
                        *expr = operand.clone();
                    } else {
                        *expr = Expr::UnaryOperations {
                            operand: expr.clone(),
                            operator: UnaryOperators::Not,
                        }
                        .into();
                    }
                    expr.equiv_convert()?;
                    *self = Expr::Eventually {
                        interval: interval.clone(),
                        not: false,
                        expr: expr.clone(),
                    };
                } else {
                    expr.equiv_convert()?;
                }

                Ok(())
            }
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
                    if let Expr::UnaryOperations {
                        operand,
                        operator: UnaryOperators::Not,
                    } = expr.as_ref()
                    {
                        *expr = operand.clone();
                    } else {
                        *expr = Expr::UnaryOperations {
                            operand: expr.clone(),
                            operator: UnaryOperators::Not,
                        }
                        .into();
                    }
                    expr.equiv_convert()?;
                    *self = Expr::Always {
                        interval: interval.clone(),
                        not: false,
                        expr: expr.clone(),
                    };
                } else {
                    expr.equiv_convert()?;
                }

                Ok(())
            }
            Expr::Unit { number, unit: _ } => {
                number.equiv_convert()?;
                *self = number.as_ref().clone();
                Ok(())
            }
            Expr::Function {
                aggregate_type,
                expr,
                bound,
            } => match aggregate_type {
                FunctionType::Count => { 
                    *aggregate_type = FunctionType::Sum;
                    *expr =  Expr::BinaryOperations { lhs: expr.clone(), rhs: Expr::Number(0).into(), operator: BinaryOperators::NotEqual }.into();
                    expr.equiv_convert()?;
                    Ok(())
                }
                FunctionType::Counttime => {                   
                    *aggregate_type = FunctionType::Sumtime;
                    *expr =  Expr::BinaryOperations { lhs: expr.clone(), rhs: Expr::Number(0).into(), operator: BinaryOperators::NotEqual }.into();
                    expr.equiv_convert()?;
                    if let Some(b) = bound {
                        b.equiv_convert()?;
                    }
                    Ok(())
                },
                FunctionType::Avgtime | FunctionType::Sumtime | FunctionType::Sum | FunctionType::Avg | FunctionType::Foreach => {
                    if let Some(b) = bound {
                        b.equiv_convert()?;
                    }
                    expr.equiv_convert()?;
                    Ok(())
                },
            },
            Expr::Interval { start, end } => {
                start.equiv_convert()?;
                end.equiv_convert()?;
                let Expr::Number(start_value) = start.as_ref() else {
                    unreachable!()
                };
                let Expr::Number(end_value) = end.as_ref() else {
                    unreachable!()
                };
                if *start_value < 0 || *end_value < 0 {
                    return Err(errors::Error::IntervalBelowZero(*start_value, *end_value).into());
                }
                if *start_value > *end_value {
                    return Err(errors::Error::IntervalStartGreaterThanEnd(
                        *start_value,
                        *end_value,
                    )
                    .into());
                }
                Ok(())
            }
            Expr::UnaryOperations { operand, operator } => {
                operand.equiv_convert()?;

                // Converts negated operand numbers to integer boolean expressions.
                if *operator == UnaryOperators::Not {
                    if let Expr::Number(number) = operand.as_ref() {
                        if *number == 0i128 {
                            *self = Expr::Number(1000)
                        } else {
                            *self = Expr::Number(0)
                        }
                    }
                // Converts unary negative operations to number expressions itself.
                } else if let Expr::Number(number) = operand.as_ref(){
                    *self = Expr::Number(number*-1)
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
