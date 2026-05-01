
use crate::{
    monitor_setup::operation_types::{AggregateType, ExprLTL, Operation, PropLTL}, program::{member_types::MemberType, operations::{BinaryOperators, UnaryOperators}}, utils::monitor_test_helper_func::*
};

#[test]
fn eventually_true_remove() {
    let operations: Vec<Operation> = vec![Operation::Number(1)];
    let mut program = eventually_prop_helper(operations, (0, 2));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (_, value) in result {
        assert!(value.is_empty());
    }
    assert_eq!(streams.first().unwrap().ltl, PropLTL::Eventually(true));
}

#[test]
fn eventually_false_remove() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (0, 2));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (idx, value) in result {
        if idx == 2 {
            assert!(value[0].1);
        }
    }
    assert_eq!(streams.first().unwrap().ltl, PropLTL::Eventually(true));
}

#[test]
fn eventually_true_remove_hard_challange_mode() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (2, 6));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (_, value) in result {
        assert!(value.is_empty());
    }
    assert_eq!(streams.first().unwrap().ltl, PropLTL::Eventually(false));

    let result = run_x_monitor_steps(streams, &device_stream, 5, 4);
    
    for (idx, value) in result {
        if idx == 6 {
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }
    assert_eq!(streams.first().unwrap().ltl, PropLTL::Eventually(true));
}

#[test]
fn eventually_false_not_removed() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = eventually_prop_helper(operations, (0, 5));
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 3);
    
    for (idx, value) in result {
        if idx == 2 {
            assert!(value.is_empty());
        }
    }
    assert_eq!(streams.first().unwrap().ltl, PropLTL::Eventually(false));
}

#[test]
fn always_false_unbound() {
    let operations: Vec<Operation> = vec![Operation::Number(0)];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    //
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
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
        Operation::SpawnTime,
        Operation::Number(2000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    //
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    //
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    //
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
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
        Operation::Number(55_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
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
        // Operation::Number(1_000),
        Operation::Number(10_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 1);
    
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
        Operation::Number(5500),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
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
        Operation::Number(1_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
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
        Operation::SpawnTime,
        Operation::Number(1000),
        Operation::SpawnTime
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (_, value) in result {
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (_, value) in result {
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
    let result = run_x_monitor_steps(streams, &device_stream, 0, 5);
    
    for (_, value) in result {
        assert!(value.is_empty());
    }
}


#[test]
fn always_nested_device_stack() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 6,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Binary {
            bin_op: BinaryOperators::NotEqual,
            idx_lhs: 3,
            idx_rhs: 5,
        },
        Operation::Foreach { idx: 4 },
        Operation::Number(1_000),
        Operation::Number(0),
        Operation::Number(10_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
    for (_, value) in result {
        assert!(value.is_empty());
    }
}


#[test]
fn always_nested_device_stack_false() {
    let operations: Vec<Operation> = vec![
        Operation::Binary {
            bin_op: BinaryOperators::Equal,
            idx_lhs: 1,
            idx_rhs: 6,
        },
        Operation::AggregateFunction {
            idx: 2,
            function_type: AggregateType::Sum,
        },
        Operation::Binary {
            bin_op: BinaryOperators::NotEqual,
            idx_lhs: 3,
            idx_rhs: 5,
        },
        Operation::Foreach { idx: 4 },
        Operation::Number(1_000),
        Operation::Number(0),
        Operation::Number(1_000),
    ];
    let mut program = always_prop_helper(operations, None);
    let device_stream = ten_device_stream();
    let Some(streams) = &mut program.environment else {
        panic!()
    };
    let result = run_x_monitor_steps(streams, &device_stream, 0, 10);
    
    for (_, value) in result {
        assert!(value[0].1);
    }
}

#[test]
fn time_behaviour_test() {
    let operations = { 
        use Operation::*;
        vec![
            Binary { bin_op: BinaryOperators::Or, idx_lhs: 1, idx_rhs: 7, },
            Unary { un_op: UnaryOperators::Not, idx: 2, },
            Binary { bin_op: BinaryOperators::Equal, idx_lhs: 3, idx_rhs: 6, },
            Binary { bin_op: BinaryOperators::Mod, idx_lhs: 4, idx_rhs: 5, },
            SpawnTime,
            Number( 24000, ),
            Number( 0, ),
            LTLBounded { bound: ( 0, 24, ), idx: 8, not: false, ltl_type: ExprLTL::Always, },
            Binary { bin_op: BinaryOperators::LessEqual, idx_lhs: 9, idx_rhs: 12, },
            TimeFunction {
                idx: 10,
                function_type: AggregateType::Sum,
                history: Vec::new(),
                bound: 24,
            },
            AggregateFunction {
                idx: 11,
                function_type: AggregateType::Sum,
            },
            Number(1000,),
            Number(24000,),
        ] 
    };
    // Always t%24s = 0S -> always[0,24] sumtime(1) < 24s;
    let program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let streams = &mut program.environment.unwrap();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    for (idx, value) in result {
        if idx == 24 || idx == 48 || idx == 72 || idx == 96{
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }
    // Test for violation at 23_000
    //Reset the property and set number as 23
    streams[0].operations[12] = Operation::Number(23_000);
    streams[0].operations[9] = Operation::TimeFunction { idx: 10, function_type: AggregateType::Sum, history: Vec::new(), bound: 24,};
    streams[0].time_verdicts.clear();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    for (idx, value) in result {
        if idx == 23 || idx == 47 || idx == 71 || idx == 95{
            assert!(value[0].1);
        } else {
            assert!(value.is_empty());
        }
    }

    // Test no violation
    streams[0].operations[12] = Operation::Number(25_000);
    streams[0].operations[9] = Operation::TimeFunction { idx: 10, function_type: AggregateType::Sum, history: Vec::new(), bound: 24,};
    streams[0].time_verdicts.clear();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    
    for (_, value) in result { assert!(value.is_empty()) }

}


#[test]
fn eventually_expr_true() {
    let operations = vec![
        Operation::LTLBounded { bound: (1,1), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) },
        Operation::Number(1_000)
    ];
    let program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let streams = &mut program.environment.unwrap();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    
    for (_, value) in result {
        assert!(value.is_empty());
    }
}

#[test]
fn eventually_expr_false() {
    let operations = vec![
            Operation::LTLBounded { bound: (1,1), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new()) },
            Operation::Number(0)
        ] ;
    let program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let streams = &mut program.environment.unwrap();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    
    for (idx, value) in result {
        if idx == 0{
            assert!(value.is_empty());
        } else {
            assert!(value[0].1);
        }
    }
}

#[test]
fn eventually_expr_time_true() {
    let operations = {
        use Operation::*;
        vec![
            LTLBounded { bound: ( 2, 5 ), idx: 1, not: false, ltl_type: ExprLTL::Eventually(Vec::new())},
            Binary {bin_op: BinaryOperators::NotEqual, idx_lhs: 2,idx_rhs: 3},
            SpawnTime,
            Number(2_000),
        ]};
        
    let program = always_prop_helper(operations, None);
    let device_stream = single_device_stream();
    let streams = &mut program.environment.unwrap();
    let result = run_x_monitor_steps(streams, &device_stream, 0, 100);
    

    for idx in (0..100).filter(|&num| !(4..7).contains(&num) ) {
        let value = result.get(&idx).unwrap();
        assert!(value.is_empty());
    }
}


