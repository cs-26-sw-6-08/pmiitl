use std::error::Error;

use crate::{errors, program::{
    expressions::Expr,
    operations::{BinaryOperators, UnaryOperators},
}};

pub fn conversion_binary_operations(
    lhs: Expr,
    rhs: Expr,
    operator: BinaryOperators,
) -> Result<Expr, Box<dyn Error>> {
    match operator {
        BinaryOperators::Times => match (lhs, rhs) {
            /* n * m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n * m)),
            /* b * b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Number((if p { 1 } else { 0 }) * (if q { 1 } else { 0 }))),
            /* n * b | b * n */
            (Expr::Number(n), Expr::Boolean(b))
            | (Expr::Boolean(b), Expr::Number(n)) => {
                Ok(Expr::Number(n * if b { 1 } else { 0 }))
            }
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Times, lhs, rhs).into()),
        },
        BinaryOperators::Divide => match (lhs, rhs) {
            /* n / m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n / m)),
            /* b / b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Number((if p { 1 } else { 0 }) / (if q { 1 } else { 0 }))),
            /* n / b */
            (Expr::Number(n), Expr::Boolean(b)) => {
                Ok(Expr::Number(n / if b { 1 } else { 0 }))
            }
            /* b / n */
            (Expr::Boolean(b), Expr::Number(n)) => {
                Ok(Expr::Number(if b { 1 } else { 0 } / n))
            }
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Divide, lhs, rhs).into()),
        },
        BinaryOperators::Plus => match (lhs, rhs) {
            /* n + m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n + m)),
            /* b + b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Number((if p { 1 } else { 0 }) + (if q { 1 } else { 0 }))),
            /* n + b | b + n*/
            (Expr::Number(n), Expr::Boolean(b))
            | (Expr::Boolean(b), Expr::Number(n)) => {
                Ok(Expr::Number(n + if b { 1 } else { 0 }))
            }
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Plus, lhs, rhs).into()),
        },
        BinaryOperators::Minus => match (lhs, rhs) {
            /* n - m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n - m)),
            /* b - b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Number((if p { 1 } else { 0 }) - (if q { 1 } else { 0 }))),
            /* n - b */
            (Expr::Number(n), Expr::Boolean(b)) => {
                Ok(Expr::Number(n - if b { 1 } else { 0 }))
            }
            /* b - n */
            (Expr::Boolean(b), Expr::Number(n)) => {
                Ok(Expr::Number(if b { 1 } else { 0 } - n))
            }
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Minus, lhs, rhs).into()),
        },
        BinaryOperators::Mod => match (lhs, rhs) {
            /* n % m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n % m)),
            /* b % b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Number((if p { 1 } else { 0 }) % (if q { 1 } else { 0 }))),
            /* n % b */
            (Expr::Number(n), Expr::Boolean(b)) => {
                Ok(Expr::Number(n % if b { 1 } else { 0 }))
            }
            /* b % n */
            (Expr::Boolean(b), Expr::Number(n)) => {
                Ok(Expr::Number(if b { 1 } else { 0 } % n))
            }
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Mod, lhs, rhs).into()),
        },
        BinaryOperators::And => match (lhs, rhs) {
            /* b & n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b && n != 0)),
            /* n & b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean(n != 0 && b)),
            /* p & q */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p && q)),
            /* n & m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n != 0 && m != 0)),

            /* p & q => !(!p || !q) */
            (lhs, rhs) => Ok(Expr::UnaryOperations {
                operand: Expr::BinaryOperations {
                    lhs: Expr::UnaryOperations {
                        operand: lhs.into(),
                        operator: UnaryOperators::Not,
                    }
                    .into(),
                    rhs: Expr::UnaryOperations {
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
        BinaryOperators::Or => match (lhs, rhs) {
            /* p | q */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p || q)),
            /* b | n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b || n != 0)),
            /* n | b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean(n != 0 || b)),
            /* n | m */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n != 0 || m != 0)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Or, lhs, rhs).into()),
        },
        BinaryOperators::Equal => match (lhs, rhs) {
            /* p = q */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p == q)),
            /* n = n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n == m)),
            /* b = n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b == (n != 0))),
            /* n = b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) == b)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Equal, lhs, rhs).into()),
        },
        BinaryOperators::NotEqual => match (lhs, rhs) {
            /* p != q */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p != q)),
            /* n != n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n != m)),
            /* b != n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b != (n != 0))),
            /* n != b*/
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) != b)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::NotEqual, lhs, rhs).into()),
        },
        BinaryOperators::Greater => match (lhs, rhs) {
            /* n > n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n > m)),
            /* n > b */
            #[allow(clippy::bool_comparison)]
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) > b)),
            /* b > n */
            #[allow(clippy::bool_comparison)]
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b > (n != 0))),
            /* b > b */
            #[allow(clippy::bool_comparison)]
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p > q)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Greater, lhs, rhs).into()),
        },
        BinaryOperators::GreaterEqual => match (lhs, rhs) {
            /* n >= n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n >= m)),
            /* n >= b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) >= b)),
            /* b >= n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b >= (n != 0))),
            /* b >= b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p >= q)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::GreaterEqual, lhs, rhs).into()),
        },
        BinaryOperators::Less => match (lhs, rhs) {
            /* n < n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n < m)),
            /* n < b */
            #[allow(clippy::bool_comparison)]
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) < b)),
            /* b < n */
            #[allow(clippy::bool_comparison)]
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b < (n != 0))),
            /* b < b */
            #[allow(clippy::bool_comparison)]
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p < q)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Less, lhs, rhs).into()),
        },
        BinaryOperators::LessEqual => match (lhs, rhs) {
            /* n <= n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n <= m)),
            /* b <= n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(b <= (n != 0))),
            /* n <= b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean((n != 0) <= b)),
            /* b <= b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(p <= q)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::LessEqual, lhs, rhs).into()),
        },
        BinaryOperators::Implies => match (lhs, rhs) {
            /* b -> b */
            (Expr::Boolean(p), Expr::Boolean(q)) => Ok(Expr::Boolean(!p || q)),
            /* b -> n */
            (Expr::Boolean(b), Expr::Number(n)) => Ok(Expr::Boolean(!b || (n != 0))),
            /* n -> b */
            (Expr::Number(n), Expr::Boolean(b)) => Ok(Expr::Boolean(n == 0 || b)),
            /* n -> n */
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Boolean(n == 0 || (m != 0))),
             /* p -> q => !p || q */
            (lhs, rhs) => Ok(Expr::BinaryOperations { lhs: Expr::UnaryOperations { operand: lhs.into(), operator: UnaryOperators::Not }.into(), rhs: rhs.into(), operator: BinaryOperators::Or }),
        },
    }
}

