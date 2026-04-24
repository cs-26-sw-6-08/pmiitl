use std::collections::HashMap;

use crate::{monitor::{streams::{IoTStream, PropertyStream}, types::Verdict}, monitor_setup::operation_types::{Operation, PropLTL}, program::Program};

pub fn run_x_monitor_steps<'a>(env: &'a mut [PropertyStream], device_stream: &'a IoTStream, t_start: i128, step_amount: i128) -> HashMap<i128, Vec<(usize, bool)>> {
    let mut result: HashMap<i128, Vec<(usize, bool)>> = HashMap::new();
    for k in t_start..(step_amount+t_start) {
        result.insert(k, vec![]);
        let cur = result.get_mut(&k).unwrap();
        for el in Program::monitor_logic(env, &(k), device_stream) {
            let (prop_num, is_violated) = el.unwrap();
            cur.push((prop_num,is_violated));
        }
    }
    result
}

pub fn eventually_prop_helper(operations: Vec<Operation>, bound: (i128,i128))-> Program {
    Program{ expressions: vec![], environment: Some(vec![PropertyStream{
        ltl: PropLTL::Eventually(false),
        bound: Some(bound),
        time_verdicts: vec![],
        operations,
    }]) }
}

pub fn always_prop_helper(operations: Vec<Operation>, bound: Option<(i128,i128)>)-> Program {
    Program{ expressions: vec![], environment: Some(vec![PropertyStream{
        ltl: PropLTL::Always,
        bound,
        time_verdicts: vec![],
        operations,
    }]) }
}

pub fn single_device_stream()->IoTStream{
    let temp_iot_stream: IoTStream = (
            vec![ ("Roomba".into(), 5_000).into() ]
        ).into();
        temp_iot_stream
}

pub fn two_device_stream()->IoTStream{
    let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba".into(), 5_000).into(),
                ("christian".into(), 1_0000).into(),
            ]
        ).into();
    temp_iot_stream
}

pub fn ten_device_stream() -> IoTStream {
    let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba0".into(), 1_000).into(),
                ("Roomba1".into(), 2_000).into(),
                ("Roomba2".into(), 3_000).into(),
                ("Roomba3".into(), 4_000).into(),
                ("Roomba4".into(), 5_000).into(),
                ("Roomba5".into(), 6_000).into(),
                ("Roomba6".into(), 7_000).into(),
                ("Roomba7".into(), 8_000).into(),
                ("Roomba8".into(), 9_000).into(),
                ("Roomba9".into(), 10_000).into(),
            ]
        ).into();
    temp_iot_stream
}