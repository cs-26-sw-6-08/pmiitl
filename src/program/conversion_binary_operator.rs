use std::error::Error;

use crate::{errors, program::{
    expressions::ExprKind,
    operations::{BinaryOperators, UnaryOperators},
    units::Unit,
}};

pub fn conversion_binary_operations(
    lhs: ExprKind,
    rhs: ExprKind,
    operator: &BinaryOperators,
) -> Result<ExprKind, Box<dyn Error>> {
    match operator {
        BinaryOperators::Times => match (&lhs, &rhs) {
            /* Ws * n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Number(m),
            )
            | (
                ExprKind::Number(m),
                ExprKind::Unit {
                    number,
                    unit: Unit::WattSeconds,
                },
            ) => match number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(*n * m).into(),
                    unit: Unit::WattSeconds,
                }),
                _ => unreachable!(),
            },
            /* s * n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::Seconds,
                },
                ExprKind::Number(m),
            )
            | (
                ExprKind::Number(m),
                ExprKind::Unit {
                    number,
                    unit: Unit::Seconds,
                },
            ) => match number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n * m).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* s * W */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Seconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Watt,
                },
            )
            | (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Watt,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Seconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 * n2).into(),
                    unit: Unit::WattSeconds,
                }),
                _ => unreachable!(),
            },
            /* W * n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::Watt,
                },
                ExprKind::Number(m),
            )
            | (
                ExprKind::Number(m),
                ExprKind::Unit {
                    number,
                    unit: Unit::Watt,
                },
            ) => match number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n * m).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!(),
            },
            /* n * m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Number(n * m)),
            /* n * b | b * n */
            (ExprKind::Number(n), ExprKind::Boolean(b))
            | (ExprKind::Boolean(b), ExprKind::Number(n)) => {
                Ok(ExprKind::Number(n * if *b { 1 } else { 0 }))
            }
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Times, lhs, rhs).into()),
        },
        BinaryOperators::Divide => match (&lhs, &rhs) {
            /* Ws / n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Number(m),
            ) => match &number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(*n / m).into(),
                    unit: Unit::WattSeconds,
                }),
                _  => unreachable!(),
            },
            /* Ws / s */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Seconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(*n1 / *n2).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!(),
            },
            /* Ws / W */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Watt,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 / *n2).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* s / n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::Seconds,
                },
                ExprKind::Number(m),
            ) => match number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n / m).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* W / n */
            (
                ExprKind::Unit {
                    number,
                    unit: Unit::Watt,
                },
                ExprKind::Number(m),
            ) => match number.as_ref() {
                ExprKind::Number(n) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n / m).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!()
            },
            /* n / m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Number(n / m)),
            /* n / b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => {
                Ok(ExprKind::Number(n / if *b { 1 } else { 0 }))
            }
            /* b / n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => {
                Ok(ExprKind::Number(if *b { 1 } else { 0 } / n))
            }
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Divide, lhs, rhs).into()),
        },
        BinaryOperators::Plus => match (&lhs, &rhs) {
            /* Ws + Ws */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::WattSeconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(*n1 + *n2).into(),
                    unit: Unit::WattSeconds,
                }),
                _ => unreachable!(),
            },
            /* s + s */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Seconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Seconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 + n2).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* W + W */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Watt,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Watt,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 + n2).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!(),
            },
            /* n + m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Number(n + m)),
            /* n + b | b + n*/
            (ExprKind::Number(n), ExprKind::Boolean(b))
            | (ExprKind::Boolean(b), ExprKind::Number(n)) => {
                Ok(ExprKind::Number(n + if *b { 1 } else { 0 }))
            }
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Plus, lhs, rhs).into()),
        },
        BinaryOperators::Minus => match (&lhs, &rhs) {
            /* Ws - Ws */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::WattSeconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(*n1 - *n2).into(),
                    unit: Unit::WattSeconds,
                }),
                _ => unreachable!(),
            },
            /* s - s */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Seconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Seconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 - n2).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* W - W */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Watt,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Watt,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 - n2).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!(),
            },
            /* n - m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Number(n - m)),
            /* n - b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => {
                Ok(ExprKind::Number(n - if *b { 1 } else { 0 }))
            }
            /* b - n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => {
                Ok(ExprKind::Number(if *b { 1 } else { 0 } - n))
            }
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Minus, lhs, rhs).into()),
        },
        BinaryOperators::Mod => match (&lhs, &rhs) {
            /* W % W */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Watt,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Watt,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 % n2).into(),
                    unit: Unit::Watt,
                }),
                _ => unreachable!(),
            },
            /* s % s */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::Seconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::Seconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 % n2).into(),
                    unit: Unit::Seconds,
                }),
                _ => unreachable!(),
            },
            /* Ws % Ws */
            (
                ExprKind::Unit {
                    number: number1,
                    unit: Unit::WattSeconds,
                },
                ExprKind::Unit {
                    number: number2,
                    unit: Unit::WattSeconds,
                },
            ) => match (number1.as_ref(), number2.as_ref()) {
                (ExprKind::Number(n1), ExprKind::Number(n2)) => Ok(ExprKind::Unit {
                    number: ExprKind::Number(n1 % n2).into(),
                    unit: Unit::WattSeconds,
                }),
                _ => unreachable!(),
            },
            /* n % m */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Number(n % m)),
            /* n % b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => {
                Ok(ExprKind::Number(n % if *b { 1 } else { 0 }))
            }
            /* b % n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => {
                Ok(ExprKind::Number(if *b { 1 } else { 0 } % n))
            }
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Mod, lhs, rhs).into()),
        },
        BinaryOperators::And => match (lhs, rhs) {
            /* b & n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(b && n != 0)),
            /* n & b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean(n != 0 && b)),
            /* p & q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Ok(ExprKind::Boolean(p && q)),

            /* p & q => !(!p || !q) */
            (lhs, rhs) => Ok(ExprKind::UnaryOperations {
                operand: ExprKind::BinaryOperations {
                    lhs: ExprKind::UnaryOperations {
                        operand: lhs.into(),
                        operator: UnaryOperators::Not,
                    }
                    .into(),
                    rhs: ExprKind::UnaryOperations {
                        operand: rhs.into(),
                        operator: UnaryOperators::Not,
                    }
                    .into(),
                    operator: BinaryOperators::Or,
                }
                .into(),
                operator: UnaryOperators::Not,
            }),
        },
        BinaryOperators::Or => match (&lhs, &rhs) {
            /* p | q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Ok(ExprKind::Boolean(*p || *q)),
            /* b | n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b || *n != 0)),
            /* n | b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean(*n != 0 || *b)),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Or, lhs, rhs).into()),
        },
        BinaryOperators::Equal => match (&lhs, &rhs) {
            /* p = q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Ok(ExprKind::Boolean(p == q)),
            /* n = n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n == m)),
            /* b = n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b == (*n != 0))),
            /* n = b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) == *b)),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Equal, lhs, rhs).into()),
        },
        BinaryOperators::NotEqual => match (&lhs, &rhs) {
            /* p != q */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Ok(ExprKind::Boolean(p != q)),
            /* n != n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n != m)),
            /* b != n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b != (*n != 0))),
            /* n != b*/
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) != *b)),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::NotEqual, lhs, rhs).into()),
        },
        BinaryOperators::Greater => match (&lhs, &rhs) {
            /* n > n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n > m)),
            /* n > b */
            #[allow(clippy::bool_comparison)]
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) > *b)),
            /* b > n */
            #[allow(clippy::bool_comparison)]
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b > (*n != 0))),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Greater, lhs, rhs).into()),
        },
        BinaryOperators::GreaterEqual => match (&lhs, &rhs) {
            /* n >= n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n >= m)),
            /* n >= b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) >= *b)),
            /* b >= n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b >= (*n != 0))),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::GreaterEqual, lhs, rhs).into()),
        },
        BinaryOperators::Less => match (&lhs, &rhs) {
            /* n < n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n < m)),
            /* n < b */
            #[allow(clippy::bool_comparison)]
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) < *b)),
            /* b < n */
            #[allow(clippy::bool_comparison)]
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b < (*n != 0))),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Less, lhs, rhs).into()),
        },
        BinaryOperators::LessEqual => match (&lhs, &rhs) {
            /* n <= n */
            (ExprKind::Number(n), ExprKind::Number(m)) => Ok(ExprKind::Boolean(n <= m)),
            /* b <= n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(*b <= (*n != 0))),
            /* n <= b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean((*n != 0) <= *b)),
            _ => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::LessEqual, lhs, rhs).into()),
        },
        BinaryOperators::Implies => match (lhs, rhs) {
            /* b -> b */
            (ExprKind::Boolean(p), ExprKind::Boolean(q)) => Ok(ExprKind::Boolean(!p || q)),
            /* b -> n */
            (ExprKind::Boolean(b), ExprKind::Number(n)) => Ok(ExprKind::Boolean(!b || (n != 0))),
            /* n -> b */
            (ExprKind::Number(n), ExprKind::Boolean(b)) => Ok(ExprKind::Boolean(n == 0 || b)),
             /* p -> q => !p || q */
            (lhs, rhs) => Ok(ExprKind::BinaryOperations { lhs: ExprKind::UnaryOperations { operand: lhs.into(), operator: UnaryOperators::Not }.into(), rhs: rhs.into(), operator: BinaryOperators::Or }),
        },
    }
}