use crate::program::{
    Program,
    expressions::Expr,
    function_types::FunctionType,
    member_types::MemberType,
    operations::{BinaryOperators, UnaryOperators},
    units::Unit,
};

#[test]
fn eventually() {
    let program = Program::new("eventually 1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Eventually {
            interval: None,
            not: false,
            expr: Expr::Number(1000).into()
        }
    );
}

#[test]
fn always() {
    let program = Program::new("always -1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::UnaryOperations { operand: Expr::Number(1000).into(), operator: UnaryOperators::Negative }.into()
        }
    );
}

#[test]
fn eventually_interval() {
    let program = Program::new("eventually[1s,3h] -1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Eventually {
            interval: Some(
                Expr::Interval {
                    start: Expr::Unit {
                        number: Expr::Number(1000).into(),
                        unit: Unit::Seconds
                    }
                    .into(),
                    end: Expr::Unit {
                        number: Expr::Number(3000).into(),
                        unit: Unit::Hours
                    }
                    .into()
                }
                .into()
            ),
            not: false,
            expr: Expr::UnaryOperations { operand: Expr::Number(1000).into(), operator: UnaryOperators::Negative }.into()
        }
    );
}

#[test]
fn always_interval() {
    let program = Program::new("always[1s,3h] -1;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: Some(
                Expr::Interval {
                    start: Expr::Unit {
                        number: Expr::Number(1000).into(),
                        unit: Unit::Seconds
                    }
                    .into(),
                    end: Expr::Unit {
                        number: Expr::Number(3000).into(),
                        unit: Unit::Hours
                    }
                    .into()
                }
                .into()
            ),
            not: false,
            expr: Expr::UnaryOperations { operand: Expr::Number(1000).into(), operator: UnaryOperators::Negative }.into()
        }
    );
}

#[test]
fn current_time() {
    let program = Program::new("always t;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::CurrentTime.into()
        }
    );
}

#[test]
fn power_unit() {
    let program = Program::new("always 5W;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Unit { number: Expr::Number(5000).into(), unit: Unit::Watt }.into()
        }
    );
}

#[test]
fn until() {
    let program = Program::new("until(1,2);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Until {
            interval: None,
            not: false,
            lhs: Expr::Number(1000).into(),
            rhs: Expr::Number(2000).into()
        }
    );
}

#[test]
fn greater() {
    let program = Program::new("always 1 > 5;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::BinaryOperations {
                lhs: Expr::Number(1000).into(),
                rhs: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::UnaryOperations {
                operand: Expr::Number(5000).into(),
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
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::UnaryOperations {
                operand: Expr::Number(5000).into(),
                operator: UnaryOperators::Negative
            }
            .into()
        }
    );
}

#[test]
fn sum() {
    let program = Program::new("always sum(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Sum,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn avg() {
    let program = Program::new("always avg(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Avg,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn count() {
    let program = Program::new("always count(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Count,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn sumtime() {
    let program = Program::new("always sumtime(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Sumtime,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn avgtime() {
    let program = Program::new("always avgtime(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Avgtime,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn counttime() {
    let program = Program::new("always counttime(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Counttime,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn foreach() {
    let program = Program::new("always foreach(5);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Foreach,
                expr: Expr::Number(5000).into()
            }
            .into()
        }
    );
}

#[test]
fn bool() {
    let program = Program::new("always true;").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Boolean(true).into()
        }
    );
}

#[test]
fn string() {
    let program = Program::new("always count(name=fridge);").unwrap();
    assert_eq!(
        program.expressions.first().unwrap().expr,
        Expr::Always {
            interval: None,
            not: false,
            expr: Expr::Function {
                aggregate_type: FunctionType::Count,
                expr: Expr::BinaryOperations {
                    lhs: Expr::Member {
                        access_type: MemberType::Name
                    }.into(),
                    rhs: Expr::String("fridge".into()).into(),
                    operator: BinaryOperators::Equal
                }
                .into()
            }
            .into()
        }
    );
}
