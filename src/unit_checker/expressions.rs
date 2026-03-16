use std::error::Error;

use crate::{
    errors,
    program::{
        expressions::ExprKind, function_types::FunctionType, member_types::MemberType,
        operations::BinaryOperators, operations::UnaryOperators,
    },
    unit_checker::types::Type,
};

impl ExprKind {
    pub fn unit_check(&self) -> Result<Type, Box<dyn Error>> {
        match self {
            ExprKind::Number(_) => Ok(Type::Number),
            ExprKind::String(_) => Ok(Type::String),
            ExprKind::Boolean(_) => Ok(Type::Bool),
            ExprKind::CurrentTime => Ok(Type::Seconds),
            ExprKind::Unit { number, unit } => {
                let unit_type = Type::unit_type(unit);
                if number.unit_check()? == Type::Number {
                    return Ok(unit_type);
                }
                Err(errors::Error::Typechecking.into())
            }
            ExprKind::Interval { start, end } => {
                let start_type = start.unit_check()?;
                let end_type = end.unit_check()?;
                if start_type == Type::Seconds && end_type == Type::Seconds {
                    return Ok(Type::Seconds);
                }
                Err(errors::Error::Typechecking.into())
            }
            ExprKind::Always {
                interval,
                not: _,
                expr,
            } | ExprKind::Eventually {
                interval,
                not: _,
                expr,
            } => {
                if let Some(interval) = interval {
                    interval.unit_check()?;
                }

                let expr_type = expr.unit_check()?;
                match expr_type {
                    Type::Bool => Ok(Type::Bool),
                    _ => Err(errors::Error::Typechecking.into()),
                }
            },
            ExprKind::Until {
                interval,
                not: _,
                lhs,
                rhs,
            } => {if let Some(interval) = interval {
                    interval.unit_check()?;
                }

                let lhs_type = lhs.unit_check()?;
                let rhs_type = rhs.unit_check()?;
                
                match (lhs_type, rhs_type) {
                    (Type::Bool, Type::Bool) => Ok(Type::Bool),
                    _ => Err(errors::Error::Typechecking.into()),
                }},
            ExprKind::BinaryOperations { lhs, rhs, operator } => {
                let lhs_type = lhs.unit_check()?;
                let rhs_type = rhs.unit_check()?;
                match operator {
                    BinaryOperators::Less
                    | BinaryOperators::Greater
                    | BinaryOperators::LessEqual
                    | BinaryOperators::GreaterEqual => match (lhs_type, rhs_type) {
                        (Type::Number, Type::Number)
                        | (Type::Bool, Type::Number)
                        | (Type::Number, Type::Bool)
                        | (Type::Bool, Type::Bool)
                        | (Type::Watt, Type::Watt)
                        | (Type::Seconds, Type::Seconds)
                        | (Type::WattSeconds, Type::WattSeconds)
                        | (Type::Hertz, Type::Hertz) => Ok(Type::Bool),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    BinaryOperators::Equal | BinaryOperators::NotEqual => {
                        match (lhs_type, rhs_type) {
                            (Type::Number, Type::Number)
                            | (Type::Bool, Type::Number)
                            | (Type::Number, Type::Bool)
                            | (Type::Bool, Type::Bool)
                            | (Type::Watt, Type::Watt)
                            | (Type::Seconds, Type::Seconds)
                            | (Type::WattSeconds, Type::WattSeconds)
                            | (Type::Hertz, Type::Hertz)
                            | (Type::String, Type::String) => Ok(Type::Bool),
                            _ => Err(errors::Error::Typechecking.into()),
                        }
                    }
                    BinaryOperators::Plus | BinaryOperators::Minus | BinaryOperators::Mod => {
                        match (lhs_type, rhs_type) {
                            (Type::Number, Type::Number)
                            | (Type::Bool, Type::Number)
                            | (Type::Number, Type::Bool)
                            | (Type::Bool, Type::Bool) => Ok(Type::Number),
                            (Type::Watt, Type::Watt) => Ok(Type::Watt),
                            (Type::Seconds, Type::Seconds) => Ok(Type::Seconds),
                            (Type::WattSeconds, Type::WattSeconds) => Ok(Type::WattSeconds),
                            (Type::Hertz, Type::Hertz) => Ok(Type::Hertz),
                            _ => Err(errors::Error::Typechecking.into()),
                        }
                    }
                    BinaryOperators::Times => match (lhs_type, rhs_type) {
                        (Type::Number, Type::Number)
                        | (Type::Bool, Type::Number)
                        | (Type::Number, Type::Bool)
                        | (Type::Bool, Type::Bool) => Ok(Type::Number),
                        (Type::Watt, Type::Number) | (Type::Number, Type::Watt) => Ok(Type::Watt),
                        (Type::Seconds, Type::Number) | (Type::Number, Type::Seconds) => {
                            Ok(Type::Seconds)
                        }
                        (Type::WattSeconds, Type::Number)
                        | (Type::Number, Type::WattSeconds)
                        | (Type::Seconds, Type::Watt)
                        | (Type::Watt, Type::Seconds) => Ok(Type::WattSeconds),
                        (Type::Hertz, Type::Number) | (Type::Number, Type::Hertz) => {
                            Ok(Type::Hertz)
                        }
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    BinaryOperators::Divide => match (lhs_type, rhs_type) {
                        (Type::Number, Type::Number)
                        | (Type::Bool, Type::Number)
                        | (Type::Number, Type::Bool)
                        | (Type::Bool, Type::Bool)
                        | (Type::Watt, Type::Watt)
                        | (Type::Seconds, Type::Seconds)
                        | (Type::WattSeconds, Type::WattSeconds)
                        | (Type::Hertz, Type::Hertz) => Ok(Type::Number),
                        (Type::Watt, Type::Number) | (Type::WattSeconds, Type::Seconds) => {
                            Ok(Type::Watt)
                        }
                        (Type::Seconds, Type::Number) | (Type::WattSeconds, Type::Watt) => {
                            Ok(Type::Seconds)
                        }
                        (Type::WattSeconds, Type::Number) => Ok(Type::WattSeconds),
                        (Type::Hertz, Type::Number) => Ok(Type::Hertz),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    BinaryOperators::And | BinaryOperators::Or | BinaryOperators::Implies => {
                        match (lhs_type, rhs_type) {
                            (Type::Number, Type::Number)
                            | (Type::Bool, Type::Number)
                            | (Type::Number, Type::Bool)
                            | (Type::Bool, Type::Bool) => Ok(Type::Bool),
                            _ => Err(errors::Error::Typechecking.into()),
                        }
                    }
                }
            }
            ExprKind::UnaryOperations { operand, operator } => {
                let operand_type = operand.unit_check()?;

                match operator {
                    UnaryOperators::Not => match operand_type {
                        Type::Number => Ok(Type::Bool),
                        Type::Bool => Ok(Type::Bool),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    UnaryOperators::Negative => match operand_type {
                        Type::Number => Ok(Type::Number),
                        Type::Bool => Ok(Type::Number),
                        Type::Hertz => Ok(Type::Hertz),
                        Type::Seconds => Ok(Type::Seconds),
                        Type::Watt => Ok(Type::Watt),
                        Type::WattSeconds => Ok(Type::WattSeconds),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                }
            }
            ExprKind::Member { access_type } => match access_type {
                MemberType::Active => Ok(Type::Bool),
                MemberType::Power => Ok(Type::Watt),
                MemberType::Name => Ok(Type::String),
            },
            ExprKind::Function {
                aggregate_type,
                expr,
            } => {
                let expr_type = expr.unit_check()?;
                match aggregate_type {
                    FunctionType::Sum | FunctionType::Avg => match expr_type {
                        Type::String => Err(errors::Error::Typechecking.into()),
                        _ => Ok(expr_type),
                    },
                    FunctionType::Count => match expr_type {
                        Type::Number => Ok(Type::Number),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    FunctionType::Sumtime | FunctionType::Avgtime => match expr_type {
                        Type::Watt => Ok(Type::WattSeconds),
                        Type::Number => Ok(Type::Seconds),
                        Type::Hertz => Ok(Type::Number),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    FunctionType::Counttime => match expr_type {
                        Type::Number => Ok(Type::Seconds),
                        _ => Err(errors::Error::Typechecking.into()),
                    },
                    FunctionType::Foreach => todo!(),
                }
            }
        }
    }
}

/*



*/
