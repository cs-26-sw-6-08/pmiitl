use crate::program::{
    function_types::FunctionType,
    operations::{BinaryOperators, UnaryOperators},
    units::Unit,
};
use crate::utils::test_helper_func::{
    always_expr, binary_expr, custom_unit_expr,
    eventually_expr, function_expr, interval_expr, number_expr,
    unary_expr, unit_expr,
};

#[test]
fn interval() {
    let mut expr = interval_expr(
        custom_unit_expr(5_000, Unit::Minutes),
        custom_unit_expr(10_000, Unit::Minutes),
    );
    expr.unit_convert();
    assert_eq!(
        expr,
        interval_expr(
            custom_unit_expr(5_000 * 60, Unit::Seconds),
            custom_unit_expr(10_000 * 60, Unit::Seconds)
        )
    );
}

#[test]
fn always() {
    let mut expr = always_expr(unit_expr(Unit::Hours));
    expr.unit_convert();
    assert_eq!(
        expr,
        always_expr(custom_unit_expr(5_000 * 60 * 60,
            Unit::Seconds
        ))
    );
}

#[test]
fn eventually() {
    let mut expr = eventually_expr(unit_expr(Unit::KiloWatts));
    expr.unit_convert();
    assert_eq!(
        expr,
        eventually_expr(custom_unit_expr(5_000 * 1_000,
            Unit::Watt
        ))
    );
}

#[test]
fn binaryoperations() {
    let mut expr = binary_expr(unit_expr(Unit::WattHours), unit_expr(Unit::WattHours), BinaryOperators::Plus);
    expr.unit_convert();
    assert_eq!(
        expr,
        binary_expr(custom_unit_expr(5_000 * 60 * 60, Unit::WattSeconds), custom_unit_expr(5_000 * 60 * 60, Unit::WattSeconds), BinaryOperators::Plus)
    );
}

#[test]
fn unaryoperations() {
    let mut expr = unary_expr(unit_expr(Unit::WattMinutes), UnaryOperators::Negative);
    expr.unit_convert();
    assert_eq!(
        expr,
        unary_expr(custom_unit_expr(5_000 * 60, Unit::WattSeconds), UnaryOperators::Negative)
    )
}

#[test]
fn number() {
    let mut expr = number_expr();
    expr.unit_convert();
    assert_eq!(expr, number_expr())
}

#[test]
fn function() {
    let mut expr = function_expr(FunctionType::Sum, unit_expr(Unit::Minutes), None);
    expr.unit_convert();
    assert_eq!(
        expr,
        function_expr(FunctionType::Sum, custom_unit_expr(5_000 * 60, Unit::Seconds), None)
    );
}

#[test]
fn unit_minutes() {
    let mut expr = unit_expr(Unit::Minutes);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 60, Unit::Seconds)
    )
}

#[test]
fn unit_hours() {
    let mut expr = unit_expr(Unit::Hours);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 60 * 60, Unit::Seconds)
    )
}

#[test]
fn unit_kilowatts() {
    let mut expr = unit_expr(Unit::KiloWatts);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 1_000, Unit::Watt)
    )
}

#[test]
fn unit_kilowatthours() {
    let mut expr = unit_expr(Unit::KiloWattHours);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 1_000 * 60 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_watthours() {
    let mut expr = unit_expr(Unit::WattHours);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 60 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_wattminutes() {
    let mut expr = unit_expr(Unit::WattMinutes);
    expr.unit_convert();
    assert_eq!(
        expr,
        custom_unit_expr(5_000 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_seconds() {
    let mut expr = unit_expr(Unit::Seconds);
    expr.unit_convert();
    assert_eq!(
        expr,
        unit_expr(Unit::Seconds)
    )
}

#[test]
fn unit_wattseconds() {
    let mut expr = unit_expr(Unit::WattSeconds);
    expr.unit_convert();
    assert_eq!(
        expr,
        unit_expr(Unit::WattSeconds)
    )
}

#[test]
fn unit_watt() {
    let mut expr = unit_expr(Unit::Watt);
    expr.unit_convert();
    assert_eq!(
        expr,
        unit_expr(Unit::Watt)
    )
}
