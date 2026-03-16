use crate::program::{
    expressions::ExprKind,
    function_types::FunctionType,
    operations::{BinaryOperators, UnaryOperators},
    units::Unit,
};

#[test]
fn interval() {
    let expr = ExprKind::Interval {
        start: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::Minutes,
        }
        .into(),
        end: ExprKind::Unit {
            number: ExprKind::Number(10000).into(),
            unit: Unit::Minutes,
        }
        .into(),
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Interval {
            start: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60).into(),
                unit: Unit::Seconds
            }
            .into(),
            end: ExprKind::Unit {
                number: ExprKind::Number(10000 * 60).into(),
                unit: Unit::Seconds
            }
            .into(),
        }
    );
}

#[test]
fn always() {
    let expr = ExprKind::Always {
        interval: Some(
            ExprKind::Interval {
                start: ExprKind::Unit {
                    number: ExprKind::Number(5000).into(),
                    unit: Unit::Minutes,
                }
                .into(),
                end: ExprKind::Unit {
                    number: ExprKind::Number(10000).into(),
                    unit: Unit::Hours,
                }
                .into(),
            }
            .into(),
        ),
        not: false,
        expr: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::Hours,
        }
        .into(),
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Always {
            interval: Some(ExprKind::Interval {
                start: ExprKind::Unit {
                    number: ExprKind::Number(5000 * 60).into(),
                    unit: Unit::Seconds,
                }
                .into(),
                end: ExprKind::Unit {
                    number: ExprKind::Number(10000 * 60 * 60).into(),
                    unit: Unit::Seconds,
                }
                .into(),
            }
            .into()),
            not: false,
            expr: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60 * 60).into(),
                unit: Unit::Seconds
            }
            .into()
        }
    );
}

#[test]
fn eventually() {
    let expr = ExprKind::Eventually {
        interval: None,
        not: false,
        expr: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::KiloWatts,
        }
        .into(),
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Eventually {
            interval: None,
            not: false,
            expr: ExprKind::Unit {
                number: ExprKind::Number(5000 * 1000).into(),
                unit: Unit::Watt
            }
            .into()
        }
    );
}

#[test]
fn until() {
    let expr = ExprKind::Until {
        interval: None,
        not: false,
        lhs: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::KiloWattHours,
        }
        .into(),
        rhs: ExprKind::Unit {
            number: ExprKind::Number(7000).into(),
            unit: Unit::KiloWattHours,
        }
        .into(),
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Until {
            interval: None,
            not: false,
            lhs: ExprKind::Unit {
                number: ExprKind::Number(5000 * 1000 * 60 * 60).into(),
                unit: Unit::WattSeconds,
            }
            .into(),
            rhs: ExprKind::Unit {
                number: ExprKind::Number(7000 * 1000 * 60 * 60).into(),
                unit: Unit::WattSeconds,
            }
            .into()
        }
    );
}

#[test]
fn binaryoperations() {
    let expr = ExprKind::BinaryOperations {
        lhs: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::WattHours,
        }
        .into(),
        rhs: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::WattHours,
        }
        .into(),
        operator: BinaryOperators::Plus,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::BinaryOperations {
            lhs: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60 * 60).into(),
                unit: Unit::WattSeconds
            }
            .into(),
            rhs: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60 * 60).into(),
                unit: Unit::WattSeconds
            }
            .into(),
            operator: BinaryOperators::Plus
        }
    );
}

#[test]
fn unaryoperations() {
    let expr = ExprKind::UnaryOperations {
        operand: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::WattMinutes,
        }
        .into(),
        operator: UnaryOperators::Negative,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::UnaryOperations {
            operand: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60).into(),
                unit: Unit::WattSeconds
            }
            .into(),
            operator: UnaryOperators::Negative
        }
    )
}

#[test]
fn number() {
    let expr = ExprKind::Number(5000).unit_converter();
    assert_eq!(expr, ExprKind::Number(5000))
}

#[test]
fn function() {
    let expr = ExprKind::Function {
        aggregate_type: FunctionType::Sum,
        expr: ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::Minutes,
        }
        .into(),
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Function {
            aggregate_type: FunctionType::Sum,
            expr: ExprKind::Unit {
                number: ExprKind::Number(5000 * 60).into(),
                unit: Unit::Seconds
            }
            .into()
        }
    );
}

#[test]
fn unit_minutes() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::Minutes,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 60).into(),
            unit: Unit::Seconds
        }
    )
}

#[test]
fn unit_hours() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::Hours,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 60 * 60).into(),
            unit: Unit::Seconds
        }
    )
}

#[test]
fn unit_kilowatts() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::KiloWatts,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 1000).into(),
            unit: Unit::Watt
        }
    )
}

#[test]
fn unit_kilowatthours() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::KiloWattHours,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 1000 * 60 * 60).into(),
            unit: Unit::WattSeconds
        }
    )
}

#[test]
fn unit_watthours() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::WattHours,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 60 * 60).into(),
            unit: Unit::WattSeconds
        }
    )
}

#[test]
fn unit_wattminutes() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::WattMinutes,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000 * 60).into(),
            unit: Unit::WattSeconds
        }
    )
}

#[test]
fn unit_seconds() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::Seconds,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::Seconds
        }
    )
}

#[test]
fn unit_wattseconds() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::WattSeconds,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::WattSeconds
        }
    )
}

#[test]
fn unit_watt() {
    let expr = ExprKind::Unit {
        number: ExprKind::Number(5000).into(),
        unit: Unit::Watt,
    }
    .unit_converter();
    assert_eq!(
        expr,
        ExprKind::Unit {
            number: ExprKind::Number(5000).into(),
            unit: Unit::Watt
        }
    )
}
