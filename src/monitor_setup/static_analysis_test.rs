use crate::{monitor::streams::OutputStream, monitor_setup::operation_types::{AggregateType, LTL, Operation}, program::{member_types::MemberType, operations::{self, BinaryOperators}}, utils::test_helper_func::operations_vec_with_sumtime};

//(LTL, Vec<Operation>, Option<(i128, i128)>)

#[test]
fn time_bound_test() {
    let operations = operations_vec_with_sumtime();
    let stream = OutputStream::from((
        LTL::Always,
        operations.clone(),
        Some((5, 10))
    )).static_analysis();

    
    //Should be range (30, 50)
    assert_eq!(
        stream.get_operations().get(2).unwrap().clone(),
        Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::with_capacity(20), max_bound: Some(20) }
    );
    //should be range (505, 1010) => 1010 - 505 = 505
    assert_eq!(
        stream.get_operations().get(6).unwrap().clone(),
        Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), max_bound: Some(505) }
    );

    let stream = OutputStream::from((
        LTL::Always,
        operations.clone(),
        None
    )).static_analysis();

    //Should be range (25,40)
    assert_eq!(
        stream.get_operations().get(2).unwrap().clone(),
        Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::with_capacity(15), max_bound: Some(15) }
    );
    //should be range (500, 1000)
    assert_eq!(
        stream.get_operations().get(6).unwrap().clone(),
        Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), max_bound: Some(500) }
    );

    
    let mut operations_without_bound = operations.clone();
    operations_without_bound[1] = Operation::LTLAlwaysUnbounded { idx: 2 };

    let stream = OutputStream::from((
        LTL::Always,
        operations_without_bound,
        None
    )).static_analysis();

    //Should be range None
    assert_eq!(
        stream.get_operations().get(2).unwrap().clone(),
        Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::new(), max_bound: None }
    );
    //should be range (500, 1000)
    assert_eq!(
        stream.get_operations().get(6).unwrap().clone(),
        Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), max_bound: Some(500) }
    );

}

