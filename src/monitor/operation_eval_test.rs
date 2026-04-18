use crate::{monitor::{operation_eval::eval_operations, types::StackValue}, monitor_setup::operation_types::Operation, utils::test_helper_func::mock_devices};



//todo: Test constants, Aggregate functions, Ltl expressions, time functions 
#[test]
fn test_constants() {
    let mut operations = [
        Operation::Number(4000),
        Operation::String("christian".into()),
        Operation::CurrentTime,
    ];
    let (spawn_t, cur_t) = (0, 1);
    let devices = mock_devices(1);
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



}