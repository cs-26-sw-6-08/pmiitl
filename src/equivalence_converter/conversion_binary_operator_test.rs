use crate::program::{
    expressions::ExprKind,
    operations::{BinaryOperators, UnaryOperators},
};

#[test]
fn binary_and() {
    let actual = ExprKind::BinaryOperations {
        lhs: ExprKind::Number(5000).into(),
        rhs: ExprKind::Number(6000).into(),
        operator: BinaryOperators::And,
    }
    .convert().unwrap();
    let expected = ExprKind::UnaryOperations {
        operand: ExprKind::BinaryOperations {
            lhs: ExprKind::UnaryOperations {
                operand: ExprKind::Number(5000).into(),
                operator: UnaryOperators::Not,
            }
            .into(),
            rhs: ExprKind::UnaryOperations {
                operand: ExprKind::Number(6000).into(),
                operator: UnaryOperators::Not,
            }
            .into(),
            operator: BinaryOperators::Or,
        }
        .into(),
        operator: UnaryOperators::Not,
    };
    assert_eq!(actual, expected);
}

#[test]
fn binary_implies() {
    let actual = ExprKind::BinaryOperations {
        lhs: ExprKind::Number(5000).into(),
        rhs: ExprKind::Number(6000).into(),
        operator: BinaryOperators::Implies,
    }
    .convert().unwrap();
    let expected = ExprKind::BinaryOperations {
        lhs: ExprKind::UnaryOperations {
            operand: ExprKind::Number(5000).into(),
            operator: UnaryOperators::Not,
        }
        .into(),
        rhs: ExprKind::Number(6000).into(),
        operator: BinaryOperators::Or,
    };
    assert_eq!(actual, expected);
}


/*

5 test for hver operation:
- 1. lovlig unit conversion
- 2. ulovlig unit conversion
- 3. boolean boolean
- 4. number number
- 5. boolean number / number boolean
- eqv. hvis der er det. 

*/