use std::error::Error;

use crate::{errors, program::{
    expressions::Expr,
    operations::{BinaryOperators, UnaryOperators},
}};

pub fn binary_operations(
    lhs: Expr,
    rhs: Expr,
    operator: BinaryOperators,
) -> Result<Expr, Box<dyn Error>> {
    match operator {
        BinaryOperators::Times => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => {
                /*
                We have to handle multiplication untraditionally
                as decimal numbers are represented as i128
                with last 3 digits being the decimal precisions

                So to multiply two i128:
                1. extract original int of second i128
                2. extract last three digits of second i128.
                3. multiply the first i128 with the integer and fraction seperately. The fraction multiplication should also be divided with 1000
                4. then add them together
                */
                let m_int = m / 1000;
                let m_frac = m % 1000;
                //check whether multiplying will make the number too high for i128
                let temp1 = n.checked_mul(m_int).ok_or(errors::Error::BinaryOperationFail(BinaryOperators::Times, n, m_int))?;
                let temp2 = (n.checked_mul(m_frac).ok_or(errors::Error::BinaryOperationFail(BinaryOperators::Times, n, m_int))?) / 1000;

                Ok(Expr::Number(temp1 + temp2))


            },
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Times, lhs, rhs).into()),
        },
        BinaryOperators::Divide => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => {
                if m == 0 {
                    Ok(Expr::Number(0))
                } else {
                    /*
                    We have to handle multiplication untraditionally
                    as decimal numbers are represented as i128
                    with last 3 digits being the decimal precisions

                    So to multiply two i128:
                    1. extract original int of second i128
                    2. extract last three digits of second i128.
                    3. multiply the first i128 with the integer and fraction seperately. The fraction multiplication should also be divided with 1000
                    4. then add them together
                    */
                    NOT WORK TODO
                
                }
            },
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Divide, lhs, rhs).into()),
        },
        BinaryOperators::Plus => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n+m)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Plus, lhs, rhs).into()),
        },
        BinaryOperators::Minus => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number(n - m)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Minus, lhs, rhs).into()),
        },
        BinaryOperators::Mod => match (lhs, rhs) {            (Expr::Number(n), Expr::Number(m)) => {
                if m == 0 {
                    Ok(Expr::Number(0))
                } else {
                    Ok(Expr::Number(n % m))
                }
            },
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Mod, lhs, rhs).into()),
        },
        BinaryOperators::And => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n != 0 && m != 0) as i128)),

            /* To make the semantics easier p & q => !(!p || !q). 
            This should be done for all non-trivial expressions such as power & true */
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
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n != 0 || m != 0) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Or, lhs, rhs).into()),
        },
        BinaryOperators::Equal => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n == m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Equal, lhs, rhs).into()),
        },
        BinaryOperators::NotEqual => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n != m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::NotEqual, lhs, rhs).into()),
        },
        BinaryOperators::Greater => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n > m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Greater, lhs, rhs).into()),
        },
        BinaryOperators::GreaterEqual => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n >= m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::GreaterEqual, lhs, rhs).into()),
        },
        BinaryOperators::Less => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n < m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::Less, lhs, rhs).into()),
        },
        BinaryOperators::LessEqual => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n <= m) as i128)),
            (lhs, rhs) => Err(errors::Error::ConversionBinaryOperation(BinaryOperators::LessEqual, lhs, rhs).into()),
        },
        BinaryOperators::Implies => match (lhs, rhs) {
            (Expr::Number(n), Expr::Number(m)) => Ok(Expr::Number((n == 0 || (m != 0)) as i128)),
            /* To make the semantics easier p -> q => !p || q. 
            This should be done for all non-trivial expressions such as power -> true */
            (lhs, rhs) => Ok(Expr::BinaryOperations { lhs: Expr::UnaryOperations { operand: lhs.into(), operator: UnaryOperators::Not }.into(), rhs: rhs.into(), operator: BinaryOperators::Or }),
        },
    }
}

