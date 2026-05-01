use crate::{monitor::streams::PropertyStream, monitor_setup::operation_types::{AggregateType, Operation, PropLTL}, utils::test_helper_func::operations_vec_with_sumtime};

//(LTL, Vec<Operation>, Option<(i128, i128)>)

// #[test]
// fn time_bound_test() {
//     let operations = operations_vec_with_sumtime();
//     let stream = PropertyStream::from((
//         PropLTL::Always,
//         operations.clone(),
//         Some((5, 10))
//     )).static_analysis();

    
//     //Should be range (30, 50)
//     assert_eq!(
//         stream.get_operations().get(2).unwrap().clone(),
//         Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::with_capacity(20), bound: Some((30,50)) }
//     );
//     //should be range (505, 1010) => 1010 - 505 = 505
//     assert_eq!(
//         stream.get_operations().get(6).unwrap().clone(),
//         Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), bound: Some((505, 1010)) }
//     );

//     let stream = PropertyStream::from((
//         PropLTL::Always,
//         operations.clone(),
//         None
//     )).static_analysis();

//     // //Should be range (25,40)
//     // assert_eq!(
//     //     stream.get_operations().get(2).unwrap().clone(),
//     //     Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::with_capacity(15), bound: Some((25,40)) }
//     // );
//     // //should be range (500, 1000)
//     // assert_eq!(
//     //     stream.get_operations().get(6).unwrap().clone(),
//     //     Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), bound: Some((500,1000)) }
//     // );

    
//     let mut operations_without_bound = operations.clone();
//     operations_without_bound[1] = Operation::LTLAlwaysUnbounded { idx: 2 };

//     let stream = PropertyStream::from((
//         PropLTL::Always,
//         operations_without_bound,
//         None
//     )).static_analysis();

//     //Should be range None
//     assert_eq!(
//         stream.get_operations().get(2).unwrap().clone(),
//         Operation::TimeFunction { idx: 3, function_type: AggregateType::Sum, history: Vec::new(), bound: None }
//     );
//     //should be range (500, 1000)
//     assert_eq!(
//         stream.get_operations().get(6).unwrap().clone(),
//         Operation::TimeFunction { idx: 7, function_type: AggregateType::Sum, history: Vec::with_capacity(505), bound: Some((500, 1000)) }
//     );

// }

