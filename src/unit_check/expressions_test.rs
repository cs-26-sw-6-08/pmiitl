use crate::{
    program::{expressions::Expr, function_types::FunctionType, member_types::MemberType, operations::{BinaryOperators, UnaryOperators}, units::Unit}, unit_check::types::Type, utils::test_helper_func::{always_expr, binary_expr, current_time, eventually_expr, function_expr, interval_expr, member_expr, number_expr, string_expr, unary_expr, unit_expr}
};

#[test]
fn plus_minus_mod_operations() {
    for operator in [
        BinaryOperators::Plus,
        BinaryOperators::Minus,
        BinaryOperators::Mod,
    ] {
        assert_eq!(
            binary_expr(number_expr(), number_expr(), operator.clone())
                .unit_check()
                .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Watt),
                unit_expr(Unit::Watt),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Watt
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Seconds),
                unit_expr(Unit::Seconds),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Seconds,
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::WattSeconds),
                unit_expr(Unit::WattSeconds),
                operator.clone(),
            )
            .unit_check()
            .unwrap(),
            Type::WattSeconds,
        );

        assert!(binary_expr(string_expr(), string_expr(), operator.clone())
            .unit_check().is_err());
        assert!(binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            operator.clone(),
        )
        .unit_check().is_err());
        assert!(binary_expr(
            number_expr(),
            unit_expr(Unit::Watt),
            operator,
        )
        .unit_check().is_err());
    }
}

#[test]
fn less_lessequal_greater_greaterequal_operations() {
    for operator in [
        BinaryOperators::Less,
        BinaryOperators::LessEqual,
        BinaryOperators::Greater,
        BinaryOperators::GreaterEqual,
    ] {
        assert_eq!(
            binary_expr(number_expr(), number_expr(), operator.clone())
                .unit_check()
                .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Watt),
                unit_expr(Unit::Watt),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Seconds),
                unit_expr(Unit::Seconds),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Number,
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::WattSeconds),
                unit_expr(Unit::WattSeconds),
                operator.clone(),
            )
            .unit_check()
            .unwrap(),
            Type::Number,
        );

        assert!( binary_expr(string_expr(), string_expr(), operator.clone())
            .unit_check().is_err());
        assert!(binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            operator.clone(),
        )
        .unit_check().is_err());
        assert!(binary_expr(
            number_expr(),
            unit_expr(Unit::Watt),
            operator,
        )
        .unit_check().is_err());
    }
}

#[test]
fn equal_notequal_operations() {
    for operator in [BinaryOperators::Equal, BinaryOperators::NotEqual] {
        assert_eq!(
            binary_expr(number_expr(), number_expr(), operator.clone())
                .unit_check()
                .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(string_expr(), string_expr(), operator.clone())
                .unit_check()
                .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Watt),
                unit_expr(Unit::Watt),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Number
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::Seconds),
                unit_expr(Unit::Seconds),
                operator.clone()
            )
            .unit_check()
            .unwrap(),
            Type::Number,
        );
        assert_eq!(
            binary_expr(
                unit_expr(Unit::WattSeconds),
                unit_expr(Unit::WattSeconds),
                operator.clone(),
            )
            .unit_check()
            .unwrap(),
            Type::Number,
        );

        assert!(binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            operator.clone(),
        )
        .unit_check().is_err());
        assert!(binary_expr(
            number_expr(),
            unit_expr(Unit::Watt),
            operator,
        )
        .unit_check().is_err());
    }
}

#[test]
fn time_operations() {
    assert_eq!(
        binary_expr(number_expr(), number_expr(), BinaryOperators::Times)
            .unit_check()
            .unwrap(),
        Type::Number
    );
    assert_eq!(
        binary_expr(unit_expr(Unit::Watt), number_expr(), BinaryOperators::Times)
            .unit_check()
            .unwrap(),
        Type::Watt
    );
    assert_eq!(
        binary_expr(number_expr(), unit_expr(Unit::Watt), BinaryOperators::Times)
            .unit_check()
            .unwrap(),
        Type::Watt
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Seconds),
            number_expr(),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::Seconds
    );
    assert_eq!(
        binary_expr(
            number_expr(),
            unit_expr(Unit::Seconds),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::Seconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::WattSeconds),
            number_expr(),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::WattSeconds
    );
    assert_eq!(
        binary_expr(
            number_expr(),
            unit_expr(Unit::WattSeconds),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::WattSeconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::WattSeconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Seconds),
            unit_expr(Unit::Watt),
            BinaryOperators::Times
        )
        .unit_check()
        .unwrap(),
        Type::WattSeconds
    );
    
    assert!(binary_expr(string_expr(), string_expr(), BinaryOperators::Times)
            .unit_check().is_err());
    assert!(binary_expr(
            unit_expr(Unit::Seconds),
            unit_expr(Unit::Seconds),
            BinaryOperators::Times,
        )
        .unit_check().is_err());
    assert!(binary_expr(
            unit_expr(Unit::WattSeconds),
            unit_expr(Unit::Watt),
            BinaryOperators::Times,
        )
        .unit_check().is_err());
}

#[test]
fn divide_operations() {
    assert_eq!(
        binary_expr(number_expr(), number_expr(), BinaryOperators::Divide)
            .unit_check()
            .unwrap(),
        Type::Number
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Watt),
            number_expr(),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Watt
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::WattSeconds),
            unit_expr(Unit::Seconds),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Watt
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Watt),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Number
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Seconds),
            number_expr(),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Seconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::WattSeconds),
            unit_expr(Unit::Watt),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Seconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::Seconds),
            unit_expr(Unit::Seconds),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Number
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::WattSeconds),
            number_expr(),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::WattSeconds
    );
    assert_eq!(
        binary_expr(
            unit_expr(Unit::WattSeconds),
            unit_expr(Unit::WattSeconds),
            BinaryOperators::Divide
        )
        .unit_check()
        .unwrap(),
        Type::Number
    );

    assert!(binary_expr(string_expr(), string_expr(), BinaryOperators::Divide)
            .unit_check()
            .is_err());
    assert!(binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            BinaryOperators::Divide,
        )
        .unit_check()
        .is_err());
    assert!(binary_expr(
            unit_expr(Unit::Seconds),
            unit_expr(Unit::Watt),
            BinaryOperators::Divide,
        )
        .unit_check()
        .is_err());
    assert!(binary_expr(
            number_expr(),
            unit_expr(Unit::Watt),
            BinaryOperators::Divide,
        )
        .unit_check().is_err());
}

#[test]
fn and_or_implies_operations() {
	for operator in [
		BinaryOperators::And,
		BinaryOperators::Or,
		BinaryOperators::Implies,
	] {
	assert_eq!(binary_expr(number_expr(), number_expr(), operator.clone()).unit_check().unwrap(), Type::Number);
    assert_eq!(binary_expr(number_expr(), unit_expr(Unit::Watt), operator.clone()).unit_check().unwrap(), Type::Number);
    assert_eq!(binary_expr(unit_expr(Unit::Seconds), number_expr(), operator.clone()).unit_check().unwrap(), Type::Number);
    assert_eq!(binary_expr(unit_expr(Unit::WattSeconds), unit_expr(Unit::WattSeconds), operator.clone()).unit_check().unwrap(), Type::Number);
	

    assert!(binary_expr(string_expr(), string_expr(), operator.clone())
            .unit_check()
            .is_err());
    assert!(binary_expr(
            unit_expr(Unit::Watt),
            unit_expr(Unit::Seconds),
            operator.clone(),
        )
        .unit_check()
        .is_err());
    assert!(binary_expr(
            unit_expr(Unit::WattSeconds),
            unit_expr(Unit::Watt),
            operator.clone(),
        )
        .unit_check()
        .is_err());
    }
}


#[test]
fn not_operation() {
    assert_eq!(unary_expr(number_expr(), UnaryOperators::Not).unit_check().unwrap(), Type::Number);

    assert!(unary_expr(string_expr(), UnaryOperators::Not).unit_check().is_err());
    assert!(unary_expr(unit_expr(Unit::Seconds), UnaryOperators::Not).unit_check().is_err());
}


#[test]
fn negative_operation() {
    assert_eq!(unary_expr(number_expr(), UnaryOperators::Negative).unit_check().unwrap(), Type::Number);
    assert_eq!(unary_expr(unit_expr(Unit::Seconds), UnaryOperators::Negative).unit_check().unwrap(), Type::Seconds);
    assert_eq!(unary_expr(unit_expr(Unit::Watt), UnaryOperators::Negative).unit_check().unwrap(), Type::Watt);
    assert_eq!(unary_expr(unit_expr(Unit::WattSeconds), UnaryOperators::Negative).unit_check().unwrap(), Type::WattSeconds);

    assert!(unary_expr(string_expr(), UnaryOperators::Negative).unit_check().is_err());
}

#[test]
fn member() {
    assert_eq!(member_expr(MemberType::Name).unit_check().unwrap(), Type::String);
    assert_eq!(member_expr(MemberType::Power).unit_check().unwrap(), Type::Watt);
}

#[test]
fn current_time_type() {
    assert_eq!(current_time().unit_check().unwrap(), Type::Seconds);
}

#[test]
fn interval() {
    assert_eq!(interval_expr(unit_expr(Unit::Seconds), unit_expr(Unit::Seconds) ).unit_check().unwrap(), Type::Seconds);
    assert_eq!(interval_expr(binary_expr(unit_expr(Unit::Seconds), unit_expr(Unit::Seconds), BinaryOperators::Plus), unit_expr(Unit::Seconds) ).unit_check().unwrap(), Type::Seconds);

    assert!(interval_expr(unit_expr(Unit::Watt), unit_expr(Unit::Seconds) ).unit_check().is_err());
    assert!(interval_expr(number_expr(), unit_expr(Unit::Seconds) ).unit_check().is_err());
}

#[test]
fn function_sum_avg() {
    assert_eq!(function_expr(FunctionType::Sum, number_expr(), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Avg, number_expr(), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Sum, unit_expr(Unit::Watt), None).unit_check().unwrap(), Type::Watt);
    assert_eq!(function_expr(FunctionType::Avg, unit_expr(Unit::Seconds), None).unit_check().unwrap(), Type::Seconds);

    assert!(function_expr(FunctionType::Sum, string_expr(), None).unit_check().is_err());
    assert!(function_expr(FunctionType::Avg, string_expr(), None).unit_check().is_err());
}

#[test]
fn function_count() {
    assert_eq!(function_expr(FunctionType::Count, number_expr(), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Count, unit_expr(Unit::Watt), None).unit_check().unwrap(), Type::Number);
    
    assert!(function_expr(FunctionType::Count, string_expr(), None).unit_check().is_err());
}

#[test]
fn function_sumtime() {
    assert_eq!(function_expr(FunctionType::Sumtime, unit_expr(Unit::Watt), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::WattSeconds);
    assert_eq!(function_expr(FunctionType::Sumtime, number_expr(), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Seconds);
    
    assert!(function_expr(FunctionType::Sumtime, string_expr(), Some(unit_expr(Unit::Seconds))).unit_check().is_err());
    assert!(function_expr(FunctionType::Sumtime, unit_expr(Unit::WattSeconds), Some(unit_expr(Unit::Seconds))).unit_check().is_err());
    assert!(function_expr(FunctionType::Sumtime, unit_expr(Unit::Seconds), Some(unit_expr(Unit::Seconds))).unit_check().is_err());

    assert!(function_expr(FunctionType::Sumtime, unit_expr(Unit::Watt), None).unit_check().is_err());
    assert!(function_expr(FunctionType::Sumtime, unit_expr(Unit::Watt), Some(unit_expr(Unit::Watt))).unit_check().is_err());
}

#[test]
fn function_counttime(){
    assert_eq!(function_expr(FunctionType::Counttime, unit_expr(Unit::Watt), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Seconds);
    assert_eq!(function_expr(FunctionType::Counttime, number_expr(), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Seconds);

    assert!(function_expr(FunctionType::Counttime, string_expr(), Some(unit_expr(Unit::Seconds))).unit_check().is_err());
    assert!(function_expr(FunctionType::Counttime, unit_expr(Unit::WattSeconds), Some(unit_expr(Unit::Seconds))).unit_check().is_err());
    assert!(function_expr(FunctionType::Counttime, unit_expr(Unit::Seconds), Some(unit_expr(Unit::Seconds))).unit_check().is_err());

    assert!(function_expr(FunctionType::Counttime, unit_expr(Unit::Watt), None).unit_check().is_err());
    assert!(function_expr(FunctionType::Counttime, unit_expr(Unit::Watt), Some(unit_expr(Unit::Watt))).unit_check().is_err());
}

#[test]
fn function_avgtime() {
    assert_eq!(function_expr(FunctionType::Avgtime, number_expr(), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Avgtime, unit_expr(Unit::Watt), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Watt);
    assert_eq!(function_expr(FunctionType::Avgtime, unit_expr(Unit::Seconds), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::Seconds);
    assert_eq!(function_expr(FunctionType::Avgtime, unit_expr(Unit::WattSeconds), Some(unit_expr(Unit::Seconds))).unit_check().unwrap(), Type::WattSeconds);
    
    assert!(function_expr(FunctionType::Avgtime, string_expr(), None).unit_check().is_err());

    assert!(function_expr(FunctionType::Avgtime, number_expr(), Some(unit_expr(Unit::Watt))).unit_check().is_err());
}

#[test]
fn function_foreach() {
    
    assert_eq!(function_expr(FunctionType::Foreach, number_expr(), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Foreach, unit_expr(Unit::Watt), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Foreach, unit_expr(Unit::Seconds), None).unit_check().unwrap(), Type::Number);
    assert_eq!(function_expr(FunctionType::Foreach, unit_expr(Unit::WattSeconds), None).unit_check().unwrap(), Type::Number);
    
    assert!(function_expr(FunctionType::Foreach, string_expr(), None).unit_check().is_err());
}

#[test]
fn always(){
    assert_eq!(always_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::And)).unit_check().unwrap(), Type::Number);
    assert_eq!(always_expr(unit_expr(Unit::Seconds)).unit_check().unwrap(), Type::Number);
    assert_eq!(always_expr(number_expr()).unit_check().unwrap(), Type::Number);

    assert!(always_expr(string_expr()).unit_check().is_err());
}

#[test]
fn eventually(){
    assert_eq!(eventually_expr(binary_expr(number_expr(), number_expr(), BinaryOperators::And)).unit_check().unwrap(), Type::Number);
    assert_eq!(eventually_expr(unit_expr(Unit::Seconds)).unit_check().unwrap(), Type::Number);
    assert_eq!(eventually_expr(number_expr()).unit_check().unwrap(), Type::Number);

    assert!(eventually_expr(string_expr()).unit_check().is_err());
}

#[test]
fn unit_fail(){    
    assert!(Expr::Unit { number: string_expr().into(), unit: Unit::Seconds }.unit_check().is_err());
}