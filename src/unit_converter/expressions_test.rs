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
    let expr = interval_expr(
        custom_unit_expr(5000, Unit::Minutes),
        custom_unit_expr(10000, Unit::Minutes),
    )
    .unit_converter();
    assert_eq!(
        expr,
        interval_expr(
            custom_unit_expr(5000 * 60, Unit::Seconds),
            custom_unit_expr(10000 * 60, Unit::Seconds)
        )
    );
}

#[test]
fn always() {
    let expr = always_expr(unit_expr(Unit::Hours)).unit_converter();
    assert_eq!(
        expr,
        always_expr(custom_unit_expr(5000 * 60 * 60,
            Unit::Seconds
        ))
    );
}

#[test]
fn eventually() {
    let expr = eventually_expr(unit_expr(Unit::KiloWatts)).unit_converter();
    assert_eq!(
        expr,
        eventually_expr(custom_unit_expr(5000 * 1000,
            Unit::Watt
        ))
    );
}

#[test]
fn binaryoperations() {
    let expr = binary_expr(unit_expr(Unit::WattHours), unit_expr(Unit::WattHours), BinaryOperators::Plus).unit_converter();
    assert_eq!(
        expr,
        binary_expr(custom_unit_expr(5000 * 60 * 60, Unit::WattSeconds), custom_unit_expr(5000 * 60 * 60, Unit::WattSeconds), BinaryOperators::Plus)
    );
}

#[test]
fn unaryoperations() {
    let expr = unary_expr(unit_expr(Unit::WattMinutes), UnaryOperators::Negative).unit_converter();
    assert_eq!(
        expr,
        unary_expr(custom_unit_expr(5000 * 60, Unit::WattSeconds), UnaryOperators::Negative)
    )
}

#[test]
fn number() {
    let expr = number_expr().unit_converter();
    assert_eq!(expr, number_expr())
}

#[test]
fn function() {
    let expr = function_expr(FunctionType::Sum, unit_expr(Unit::Minutes)).unit_converter();
    assert_eq!(
        expr,
        function_expr(FunctionType::Sum, custom_unit_expr(5000 * 60, Unit::Seconds))
    );
}

#[test]
fn unit_minutes() {
    let expr = unit_expr(Unit::Minutes).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 60, Unit::Seconds)
    )
}

#[test]
fn unit_hours() {
    let expr = unit_expr(Unit::Hours).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 60 * 60, Unit::Seconds)
    )
}

#[test]
fn unit_kilowatts() {
    let expr = unit_expr(Unit::KiloWatts).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 1000, Unit::Watt)
    )
}

#[test]
fn unit_kilowatthours() {
    let expr = unit_expr(Unit::KiloWattHours).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 1000 * 60 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_watthours() {
    let expr = unit_expr(Unit::WattHours).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 60 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_wattminutes() {
    let expr = unit_expr(Unit::WattMinutes).unit_converter();
    assert_eq!(
        expr,
        custom_unit_expr(5000 * 60, Unit::WattSeconds)
    )
}

#[test]
fn unit_seconds() {
    let expr = unit_expr(Unit::Seconds).unit_converter();
    assert_eq!(
        expr,
        unit_expr(Unit::Seconds)
    )
}

#[test]
fn unit_wattseconds() {
    let expr = unit_expr(Unit::WattSeconds).unit_converter();
    assert_eq!(
        expr,
        unit_expr(Unit::WattSeconds)
    )
}

#[test]
fn unit_watt() {
    let expr = unit_expr(Unit::Watt).unit_converter();
    assert_eq!(
        expr,
        unit_expr(Unit::Watt)
    )
}
