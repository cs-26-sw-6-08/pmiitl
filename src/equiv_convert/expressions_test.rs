use crate::program::function_types::FunctionType;
use crate::program::operations::{BinaryOperators, UnaryOperators};
use crate::program::units::Unit;
use crate::utils::test_helper_func::{always_expr, binary_expr, custom_number_expr, custom_unit_expr, eventually_expr, function_expr, interval_expr, number_expr, unary_expr, unit_expr};

#[test]
fn count(){
    let mut expr = function_expr(FunctionType::Count, number_expr());
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, function_expr(FunctionType::Sum, number_expr()));
}

#[test]
fn counttime(){
    let mut expr = function_expr(FunctionType::Counttime, number_expr());
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, function_expr(FunctionType::Sumtime, number_expr()));
}

#[test]
fn always(){
    let mut expr = always_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, always_expr(custom_number_expr(10000)));
}

#[test]
fn eventually(){
    let mut expr = eventually_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, eventually_expr(custom_number_expr(10000)));
}

#[test]
fn interval(){
    let mut expr = interval_expr(custom_unit_expr(5000, Unit::Seconds), custom_unit_expr(10000, Unit::Seconds));
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, interval_expr(number_expr(), custom_number_expr(10000)));
}

#[test]
fn unit(){
    let mut expr = unit_expr(Unit::Seconds);
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, number_expr());
}

#[test]
fn unary() {
    let mut expr = unary_expr(unit_expr(Unit::Seconds), UnaryOperators::Negative );
    assert!(expr.equiv_convert().is_ok());
    assert_eq!(expr, unary_expr(number_expr(), UnaryOperators::Negative));
}