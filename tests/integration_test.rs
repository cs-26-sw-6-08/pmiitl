use rv_iot::{
    program::{
        Program,
        expressions::SpannedExpr,
        function_types::FunctionType,
        member_types::MemberType,
        operations::{BinaryOperators, UnaryOperators},
    },
    utils::test_helper_func::{
        always_expr, always_interval_expr, binary_expr, current_time, custom_number_expr,
        eventually_interval_expr, function_expr, interval_expr, member_expr, number_expr,
        string_expr, unary_expr,
    },
};
#[test]
fn test1() {
    let mut program =
        Program::new("always (t % 24h = 0s) -> always[0h,24h] sumtime(active * power) < 10 kWh;")
            .unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                unary_expr(
                    binary_expr(
                        binary_expr(
                            current_time(),
                            custom_number_expr(86_400_000),
                            BinaryOperators::Mod,
                        ),
                        custom_number_expr(0),
                        BinaryOperators::Equal,
                    ),
                    UnaryOperators::Not,
                ),
                always_interval_expr(
                    interval_expr(custom_number_expr(0), custom_number_expr(86_400_000)),
                    binary_expr(
                        function_expr(
                            FunctionType::Sumtime,
                            binary_expr(
                                member_expr(MemberType::Active),
                                member_expr(MemberType::Power),
                                BinaryOperators::Times,
                            ),
                        ),
                        custom_number_expr(36_000_000_000),
                        BinaryOperators::Less,
                    ),
                ),
                BinaryOperators::Or,
            )),
        }],
        environment: None
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test2() {
    let mut program = Program::new("! eventually count(active) > 5;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(unary_expr(
                binary_expr(
                    function_expr(FunctionType::Sum, member_expr(MemberType::Active)),
                    number_expr(),
                    BinaryOperators::Greater,
                ),
                UnaryOperators::Not,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test3() {
    let mut program = Program::new("always foreach(active -> eventually[0h,6h] !active);").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(function_expr(
                FunctionType::Foreach,
                binary_expr(
                    unary_expr(member_expr(MemberType::Active), UnaryOperators::Not),
                    eventually_interval_expr(
                        interval_expr(custom_number_expr(0), custom_number_expr(21_600_000)),
                        unary_expr(member_expr(MemberType::Active), UnaryOperators::Not),
                    ),
                    BinaryOperators::Or,
                ),
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test4() {
    let mut program = Program::new("always count(name=christian & active);").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(function_expr(
                FunctionType::Sum,
                unary_expr(
                    binary_expr(
                        unary_expr(
                            binary_expr(
                                member_expr(MemberType::Name),
                                string_expr(),
                                BinaryOperators::Equal,
                            ),
                            UnaryOperators::Not,
                        ),
                        unary_expr(member_expr(MemberType::Active), UnaryOperators::Not),
                        BinaryOperators::Or,
                    ),
                    UnaryOperators::Not,
                ),
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test5() {
    let mut program =
        Program::new("always count(active) >= 5 -> eventually[0h,6h] count(active) < 5;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                unary_expr(
                    binary_expr(
                        function_expr(FunctionType::Sum, member_expr(MemberType::Active)),
                        number_expr(),
                        BinaryOperators::GreaterEqual,
                    ),
                    UnaryOperators::Not,
                ),
                eventually_interval_expr(
                    interval_expr(custom_number_expr(0), custom_number_expr(21_600_000)),
                    binary_expr(
                        function_expr(FunctionType::Sum, member_expr(MemberType::Active)),
                        number_expr(),
                        BinaryOperators::Less,
                    ),
                ),
                BinaryOperators::Or,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test6() {
    let mut program = Program::new("always sum(active * power) <= 100 W;").unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![SpannedExpr {
            line: 1,
            expr: always_expr(binary_expr(
                function_expr(
                    FunctionType::Sum,
                    binary_expr(
                        member_expr(MemberType::Active),
                        member_expr(MemberType::Power),
                        BinaryOperators::Times,
                    ),
                ),
                custom_number_expr(100_000),
                BinaryOperators::LessEqual,
            )),
        }],
        environment: None,
    };

    assert_eq!(program, expected_program);
}

#[test]
fn test7() {
    let mut program = Program::new(
        "always 1; 
    ! always[0s,5s] 2; 
    eventually[0s,6s] 3; 
    ! eventually 4;",
    )
    .unwrap();

    assert!(program.unit_convert().is_ok());
    assert!(program.unit_check().is_ok());
    assert!(program.equiv_convert().is_ok());
    assert!(program.monitorability_check().is_ok());

    let expected_program = Program {
        expressions: vec![
            SpannedExpr {
                line: 1,
                expr: always_expr(custom_number_expr(1_000)),
            },
            SpannedExpr {
                line: 2,
                expr: eventually_interval_expr(
                    interval_expr(custom_number_expr(0), number_expr()),
                    custom_number_expr(0),
                ),
            },
            SpannedExpr {
                line: 3,
                expr: eventually_interval_expr(
                    interval_expr(custom_number_expr(0), custom_number_expr(6_000)),
                    custom_number_expr(3_000),
                ),
            },
            SpannedExpr {
                line: 4,
                expr: always_expr(custom_number_expr(0)),
            },
        ],
        environment: None,
    };

    assert_eq!(program, expected_program);
}
