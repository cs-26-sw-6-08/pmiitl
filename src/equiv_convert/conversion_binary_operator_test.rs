use crate::program::member_types::MemberType;
use crate::program::operations::UnaryOperators;
use crate::program::{
    expressions::Expr,
    operations::BinaryOperators,
};
use crate::utils::test_helper_func::{binary_expr, bool_expr, member_expr, number_expr, unary_expr};

#[test]
fn times() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Times).equiv_convert().unwrap(), Expr::Number(5000*5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Times).equiv_convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Times).equiv_convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Times).equiv_convert().unwrap(), Expr::Number(1));
}

#[test]
fn divide() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Divide).equiv_convert().unwrap(), Expr::Number(5000/5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Divide).equiv_convert().unwrap(), Expr::Number(5000));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Divide).equiv_convert().unwrap(), Expr::Number(1/5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Divide).equiv_convert().unwrap(), Expr::Number(1));
}

#[test]
fn plus() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Plus).equiv_convert().unwrap(), Expr::Number(5000+5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Plus).equiv_convert().unwrap(), Expr::Number(5000+1));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Plus).equiv_convert().unwrap(), Expr::Number(1+5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Plus).equiv_convert().unwrap(), Expr::Number(1+1));
}

#[test]
fn minus() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Minus).equiv_convert().unwrap(), Expr::Number(5000-5000));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Minus).equiv_convert().unwrap(), Expr::Number(5000-1));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Minus).equiv_convert().unwrap(), Expr::Number(1-5000));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Minus).equiv_convert().unwrap(), Expr::Number(1-1));
}

#[test]
fn modulo() {
    assert_eq!(binary_expr(number_expr(), number_expr(), BinaryOperators::Mod).equiv_convert().unwrap(), Expr::Number(0));
    assert_eq!(binary_expr(number_expr(), bool_expr(), BinaryOperators::Mod).equiv_convert().unwrap(), Expr::Number(0));
    assert_eq!(binary_expr(bool_expr(), number_expr(), BinaryOperators::Mod).equiv_convert().unwrap(), Expr::Number(1));
    assert_eq!(binary_expr(bool_expr(), bool_expr(), BinaryOperators::Mod).equiv_convert().unwrap(), Expr::Number(0));
}

#[test]
fn and_or_implies_eqal_greaterequal_lessequal() {
    for operator in [BinaryOperators::And, BinaryOperators::Or, BinaryOperators::Implies, BinaryOperators::Equal, BinaryOperators::GreaterEqual, BinaryOperators::LessEqual] {
        assert_eq!(binary_expr(number_expr(), number_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(number_expr(), bool_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(bool_expr(), number_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(true));
        assert_eq!(binary_expr(bool_expr(), bool_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(true));
    }
   
}

#[test]
fn and_equivalence_conversion() {
    assert_eq!(binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::And).equiv_convert().unwrap(), 
                unary_expr(binary_expr(unary_expr(number_expr(), UnaryOperators::Not), unary_expr(member_expr(MemberType::Active), UnaryOperators::Not), BinaryOperators::Or), UnaryOperators::Not));
}



#[test]
fn notequal_greater_less() {
    for operator in [BinaryOperators::NotEqual, BinaryOperators::Greater, BinaryOperators::Less] {
        assert_eq!(binary_expr(number_expr(), number_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(number_expr(), bool_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(bool_expr(), number_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(false));
        assert_eq!(binary_expr(bool_expr(), bool_expr(), operator.clone()).equiv_convert().unwrap(), Expr::Boolean(false));
    }
}

#[test]
fn implies_equivalence_conversion() {
    assert_eq!(binary_expr(number_expr(), member_expr(MemberType::Active), BinaryOperators::Implies).equiv_convert().unwrap(), 
                binary_expr(unary_expr(number_expr(), UnaryOperators::Not), member_expr(MemberType::Active), BinaryOperators::Or));
}

