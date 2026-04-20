use crate::{monitor::{operation_eval::eval_operations, streams::IoTStream, types::{StackValue, Verdict}}, monitor_setup::operation_types::{AggregateType, Operation}, program::{member_types::MemberType, operations::BinaryOperators}, utils::test_helper_func::mock_devices};



//todo: Ltl expressions, time functions, bin op, unary op, Random tests
#[test]
fn test_constants() {
    let mut operations = [
        Operation::Number(4000),
        Operation::String("christian".into()),
        Operation::CurrentTime,
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices = mock_devices(1).into();
    assert_eq!(
        StackValue::from(4000),    
        eval_operations(&mut operations[0..1], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(&"christian".into()),    
        eval_operations(&mut operations[1..2], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(0),    
        eval_operations(&mut operations[2..3], &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(1000),    
        eval_operations(&mut operations[2..3], &devices, &1, &cur_t).unwrap()
    )
}


#[test]
fn aggregate_functions() {
    let mut sum = [
        Operation::AggregateFunction { idx: 1, function_type: AggregateType::Sum },
        Operation::Member(MemberType::Power)
    ];
    let mut avg = [
        Operation::AggregateFunction { idx: 1, function_type: AggregateType::Avg },
        Operation::Member(MemberType::Power)
    ];
    let mut foreach = [
        Operation::Foreach { idx: 1 },
        Operation::Binary { bin_op: BinaryOperators::Equal, idx_lhs: 2, idx_rhs: 3 },
        Operation::Member(MemberType::Power),
        Operation::Number(10)
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices: IoTStream = mock_devices(3).into();
    let devices_power_all_10: IoTStream = mock_devices(3).into_iter().map(|mut device| { 
        device.power = 10;
        device
     }).collect::<Vec<_>>().into(); 

    assert_eq!(
        StackValue::from(30),    
        eval_operations(&mut sum, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(20),    
        eval_operations(&mut sum, &mock_devices(2).into(), &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(10),    
        eval_operations(&mut avg, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(false),    
        eval_operations(&mut foreach, &devices, &spawn_t, &cur_t).unwrap()
    );
    assert_eq!(
        StackValue::from(true),    
        eval_operations(&mut foreach, &devices_power_all_10, &spawn_t, &cur_t).unwrap()
    );
}

//todo: Overvej om LTL expression altid skal return either true or false, and whether undecided should be decided by taintness
// todo: Aske
#[test]
fn ltl_expressions() {
    let mut always_unb = [
        Operation::LTLAlwaysUnbounded { idx: 1 }, 
        Operation::Binary { bin_op: BinaryOperators::NotEqual, idx_lhs: 2, idx_rhs: 3 },
        Operation::CurrentTime, 
        Operation::Number(10000)
    ];
    let devices: IoTStream = mock_devices(3).into();
    assert_eq!(
        StackValue::from(true),
        eval_operations(&mut always_unb, &devices, &0, &10).unwrap()
    );
    assert!(
        (0..10).all(|t_spawn| eval_operations(&mut always_unb, &devices, &t_spawn, &10).unwrap() == StackValue::from(true) )
    )
}