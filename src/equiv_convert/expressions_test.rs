use crate::program::function_types::FunctionType;
use crate::utils::test_helper_func::{function_expr, number_expr};

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

/*
TODO: If we want make test for always, eventually, interval and unit.
*/