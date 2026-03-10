use crate::program::{
    Program, expressions::ExprKind, operations::BinaryOperators, operations::UnaryOperators,
    units::Unit,
};

#[test]
fn eventually() {
    let program = Program::new("eventually 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Eventually {
            interval: None,
            not: false,
            expr: ExprKind::Number(1000).into()
        }
    );
}

#[test]
fn always() {
    let program = Program::new("always 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::Number(1000).into()
        }
    );
}

#[test]
fn interval() {
    let program = Program::new("always[1s,3h] 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: Some(
                ExprKind::Interval {
                    start: ExprKind::Unit {
                        number: ExprKind::Number(1000).into(),
                        unit: Unit::Seconds
                    }
                    .into(),
                    end: ExprKind::Unit {
                        number: ExprKind::Number(3000).into(),
                        unit: Unit::Hours
                    }
                    .into()
                }
                .into()
            ),
            not: false,
            expr: ExprKind::Number(1000).into()
        }
    );
}

#[test]
fn greater() {
    let program = Program::new("always 1 > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Greater
            }
            .into()
        }
    );
}

#[test]
fn less() {
    let program = Program::new("always 1 < 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Less
            }
            .into()
        }
    );
}

#[test]
fn greater_equal() {
    let program = Program::new("always 1 >= 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::GreaterEqual
            }
            .into()
        }
    );
}

#[test]
fn less_equal() {
    let program = Program::new("always 1 <= 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::LessEqual
            }
            .into()
        }
    );
}

#[test]
fn equal() {
    let program = Program::new("always 1 = 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Equal
            }
            .into()
        }
    );
}

#[test]
fn not_equal() {
    let program = Program::new("always 1 != 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::NotEqual
            }
            .into()
        }
    );
}

#[test]
fn or() {
    let program = Program::new("always 1 | 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Or
            }
            .into()
        }
    );
}

#[test]
fn implies() {
    let program = Program::new("always 1 -> 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Implies
            }
            .into()
        }
    );
}

#[test]
fn and() {
    let program = Program::new("always 1 & 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::And
            }
            .into()
        }
    );
}

#[test]
fn add() {
    let program = Program::new("always 1 + 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Plus
            }
            .into()
        }
    );
}

#[test]
fn minus() {
    let program = Program::new("always 1 - 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Minus
            }
            .into()
        }
    );
}

#[test]
fn times() {
    let program = Program::new("always 1 * 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Times
            }
            .into()
        }
    );
}

#[test]
fn devide() {
    let program = Program::new("always 1 / 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Divide
            }
            .into()
        }
    );
}

#[test]
fn modulo() {
    let program = Program::new("always 1 % 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::BinaryOperations {
                lhs: ExprKind::Number(1000).into(),
                rhs: ExprKind::Number(5000).into(),
                operator: BinaryOperators::Mod
            }
            .into()
        }
    );
}

#[test]
fn not() {
    let program = Program::new("always !5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::UnaryOperations {
                operand: ExprKind::Number(5000).into(),
                operator: UnaryOperators::Not
            }
            .into()
        }
    );
}

#[test]
fn negative() {
    let program = Program::new("always -5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        ExprKind::Always {
            interval: None,
            not: false,
            expr: ExprKind::UnaryOperations {
                operand: ExprKind::Number(5000).into(),
                operator: UnaryOperators::Negative
            }
            .into()
        }
    );
}
