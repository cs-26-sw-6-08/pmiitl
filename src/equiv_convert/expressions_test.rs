use crate::program::function_types::FunctionType;
use crate::program::member_types::MemberType;
use crate::program::operations::UnaryOperators;
use crate::program::{
    expressions::Expr,
    operations::BinaryOperators,
};
use crate::utils::test_helper_func::{always_expr, binary_expr, custom_number_expr, function_expr, member_expr, number_expr, unary_expr};

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
