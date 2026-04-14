use crate::{monitor_setup::operation_types::{LTL, Operation}, program::{function_types::FunctionType, member_types::MemberType, operations::BinaryOperators}, utils::test_helper_func::*};
use crate::program::{operations::UnaryOperators};


#[test]
fn constant_rules() {
    let expr = custom_number_expr(10_000);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::Number(10_000)]);

    let expr = string_expr();
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::String(String::from("christian"))]);

    let expr = current_time();
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::CurrentTime]);

    let expr = member_expr(MemberType::Active);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::Member(MemberType::Active)]);
    
    let expr = member_expr(MemberType::Name);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::Member(MemberType::Name)]);
    
    let expr = member_expr(MemberType::Power);
    let yes_expr = expr.compile_expression();
    assert!(yes_expr.is_ok());
    assert_eq!(yes_expr.unwrap(), vec![Operation::Member(MemberType::Power)]);
}


#[test]
fn ltl_rules() {
    //Always Unbounded
    let num_expr = number_expr();
    let compiled_expr = always_expr(num_expr.clone()).compile_expression();
    assert!(compiled_expr.is_ok());
    assert_eq!(
        compiled_expr.unwrap().as_slice(), 
        [
            Operation::LTLAlwaysUnbounded { idx: 1 }, 
            Operation::Number(5000)
        ]
    );

    //Always bounded
    let interval = interval_expr(custom_number_expr(10), custom_number_expr(20));
    let compiled_expr = always_interval_expr(interval, num_expr.clone()).compile_expression();
    assert!(compiled_expr.is_ok());
    assert_eq!(
        compiled_expr.unwrap().as_slice(), 
        [
            Operation::LTLBounded { bound: (10, 20), idx: 1, not: false, ltl_type: LTL::Always }, 
            Operation::Number(5000)
        ]
    );
    
    //Eventually bounded
    let interval = interval_expr(custom_number_expr(10), custom_number_expr(20));
    let compiled_expr = eventually_interval_expr(interval, num_expr.clone()).compile_expression();
    assert!(compiled_expr.is_ok());
    assert_eq!(
        compiled_expr.unwrap().as_slice(), 
        [
            Operation::LTLBounded { bound: (10, 20), idx: 1, not: false, ltl_type: LTL::Eventually }, 
            Operation::Number(5000)
        ]
    );
    
    //Illegals
    let illegals = [eventually_expr(num_expr.clone()), always_negated_expr(num_expr.clone())];
    assert!(illegals.iter().all(|ill| ill.compile_expression().is_err()));
    
    
}

#[test]
fn binary_rules() {
    let all = [
        BinaryOperators::Equal,
        BinaryOperators::Less,
        BinaryOperators::LessEqual,
        BinaryOperators::GreaterEqual,
        BinaryOperators::NotEqual,
        BinaryOperators::Plus,
        BinaryOperators::Minus,
        BinaryOperators::Times,
        BinaryOperators::Divide,
        BinaryOperators::Mod,
        BinaryOperators::Or,
        BinaryOperators::Greater,
    ];
    for cur_type in all {
        let expr = binary_expr(custom_number_expr(10_000),custom_number_expr(10_000),cur_type.clone());
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        assert_eq!(yes_expr.unwrap(), vec![Operation::Binary { bin_op: cur_type, idx_lhs: 1, idx_rhs: 2 }, Operation::Number(10_000), Operation::Number(10_000)]);
    }
}

#[test]
fn unary_rules() {
    let all = [
        UnaryOperators::Not,
        UnaryOperators::Negative
    ];
    for cur_type in all {
        let expr = unary_expr(custom_number_expr(10_000),cur_type.clone());
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        assert_eq!(yes_expr.unwrap(), vec![Operation::Unary { un_op: cur_type, idx: 1 }, Operation::Number(10_000)]);
    }
}

#[test]
fn function_rules() {
    let all = [
        FunctionType::Sum,
        FunctionType::Avg,
        FunctionType::Sumtime,
        FunctionType::Avgtime,
        FunctionType::Foreach
    ];
    for cur_type in all {
        let expr = function_expr(cur_type.clone(), custom_number_expr(10_000));
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_ok());
        match cur_type.clone(){
            FunctionType::Sum => assert_eq!(yes_expr.unwrap(), vec![Operation::Sum { idx: 1 }, Operation::Number(10_000)]),
            FunctionType::Avg => assert_eq!(yes_expr.unwrap(), vec![Operation::Avg { idx: 1 }, Operation::Number(10_000)]),
            FunctionType::Sumtime => assert_eq!(yes_expr.unwrap(), vec![Operation::Sumtime { idx: 1 }, Operation::Sum{ idx: 2}, Operation::Number(10_000)]),
            FunctionType::Avgtime => assert_eq!(yes_expr.unwrap(), vec![Operation::Avgtime { idx: 1 }, Operation::Sum{ idx: 2}, Operation::Number(10_000)]),
            FunctionType::Foreach => assert_eq!(yes_expr.unwrap(), vec![Operation::Foreach { idx: 1 }, Operation::Number(10_000)]),
            _ => unreachable!()
        }
    }

}

#[test]
fn function_rules_not_allowed() {
    let all = [
        FunctionType::Count,
        FunctionType::Counttime,
    ];

    for cur_type in all {
        let expr = function_expr(cur_type.clone(), custom_number_expr(10_000));
        let yes_expr = expr.compile_expression();
        assert!(yes_expr.is_err());
    }
}

#[test]
fn large_expr() {
    let mem_name = member_expr(MemberType::Name);
    let str = string_expr();
    let bin_op_eq = binary_expr(mem_name, str, BinaryOperators::Equal);
    let mem_pow = member_expr(MemberType::Power);
    let bin_op = binary_expr(mem_pow, bin_op_eq, BinaryOperators::Times);
    let sumtime = function_expr(FunctionType::Sumtime, bin_op);
    let num = number_expr();
    let large_expr = binary_expr(sumtime, num, BinaryOperators::Less);

    assert_eq!(
        large_expr.compile_expression().unwrap(),
        [
            Operation::Binary { bin_op: BinaryOperators::Less, idx_lhs: 1, idx_rhs: 8 },
            Operation::Sumtime { idx: 2 },
            Operation::Sum { idx: 3 },
            Operation::Binary { bin_op: BinaryOperators::Times, idx_lhs: 4, idx_rhs: 5 },
            Operation::Member(MemberType::Power),
            Operation::Binary { bin_op: BinaryOperators::Equal, idx_lhs: 6, idx_rhs: 7 },
            Operation::Member(MemberType::Name),
            Operation::String("christian".to_owned()),
            Operation::Number(5_000)
        ]
    )
}