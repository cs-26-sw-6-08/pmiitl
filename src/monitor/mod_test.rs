use crate::{
    monitor_setup::operation_types::{AggregateType, LTL, Operation},
    program::{function_types::FunctionType, member_types::MemberType, operations::BinaryOperators},
    utils::monitor_test_helper_func::*,
};

use colored::Colorize;

#[test]
fn eventually_true_remove() {
    let operations: Vec<Operation> = vec![Operation::Number(1)];
    let mut program = eventually_prop_helper(operations, (0, 2));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
    assert_eq!(streams.first().unwrap().ltl, LTL::Eventually(true));
}

#[test]
fn eventually_false_remove() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (0, 2));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        if idx == 2 {
            assert!(value[0].1);
        }
    }
    assert_eq!(streams.first().unwrap().ltl, LTL::Eventually(true));
}

#[test]
fn eventually_true_remove_hard_challange_mode() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (2, 6));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
    assert_eq!(streams.first().unwrap().ltl, LTL::Eventually(false));

    let result = run_x_monitor_steps(streams, &device_stream, &5, 4);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        if idx == 6 {
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }
    assert_eq!(streams.first().unwrap().ltl, LTL::Eventually(true));
}

#[test]
fn eventually_false_not_removed() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (0, 5));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 3);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        if idx == 2 {
            assert!(value.is_empty());
        }
    }
    assert_eq!(streams.first().unwrap().ltl, LTL::Eventually(false));
}

#[test]
fn always_false_unbound() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 100);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value[0].1);
    }
}

#[test]
fn always_true_unbound() {
    let operations: Vec<Operation> = vec![Operation::Number(1)];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 100);
    //println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_false_bound() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = always_prop_helper(operations, Some((0, 5)));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        if idx <= 5 {
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }
}

#[test]
fn always_t_mod_switch() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Mod,
            idx_lhs: 1,
            idx_rhs: 2,
        },
        Operation::CurrentTime,
        Operation::Number(2000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 100);
    //println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        if idx % 2 == 0 {
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }
}

#[test]
fn always_simple_count_true() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Number(1000),
        Operation::Number(10000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 100);
    //println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_simple_count_false() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Number(1000),
        Operation::Number(1000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 100);
    //println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value[0].1);
    }
}

#[test]
fn always_simple_sum_member_true() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Member(MemberType::Power),
        Operation::Number(5000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_simple_sum_member_true2() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Member(MemberType::Power),
        Operation::Number(550000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_simple_sum_member_false() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Member(MemberType::Power),
        Operation::Number(10_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value[0].1);
    }
}

#[test]
fn always_simple_avg_member_true() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Avg,
        },
        Operation::Member(MemberType::Power),
        Operation::Number(55000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_simple_avg_member_false() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 3,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Avg,
        },
        Operation::Member(MemberType::Power),
        Operation::Number(1000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 10);
    println!("{}", format!("{:#?}", result).green());
    for (_, value) in result {
        assert!(value[0].1);
    }
}

#[test]
fn always_mul_check() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        Operation::Binary {
            bin_op: BinaryOperators::Times,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        Operation::CurrentTime,
        Operation::Number(1000),
        Operation::CurrentTime
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_div_check() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 5,
        },
        Operation::Binary {
            bin_op: BinaryOperators::Divide,
            idx_lhs: 2,
            idx_rhs: 4,
        },
        Operation::AggregateFunction { idx: 3, function_type: AggregateType::Sum },
        Operation::Member(MemberType::Power),
        Operation::Number(2_000),
        Operation::Number(2_500)
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn always_minus_check() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 4,
        },
        Operation::Binary {
            bin_op: BinaryOperators::Minus,
            idx_lhs: 2,
            idx_rhs: 3,
        },
        Operation::Number(2_000),
        Operation::Number(1_000),
        Operation::Number(1_000)
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, &0, 5);
    println!("{}", format!("{:#?}", result).green());
    for (idx, value) in result {
        assert!(value.is_empty());
    }
}