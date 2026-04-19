use std::collections::HashMap;

use crate::{monitor::streams::{IoTStream, OutputStream}, monitor_setup::operation_types::{LTL, Operation}, program::Program};



pub fn run_x_monitor_steps<'a>(env: &'a mut [OutputStream], device_stream: &'a IoTStream, t_start: &'a i128, step_amount: i128) -> HashMap<i128, Vec<(usize, bool)>> {
    let mut result: HashMap<i128, Vec<(usize, bool)>> = HashMap::new();
    for k in 0..step_amount {
        let current_time = t_start + k;
        println!("{current_time}");
        result.insert(t_start+k, vec![]);
        let cur = result.get_mut(&current_time).unwrap();
        for (prop_num, is_violated) in Program::monitor_logic(env, &(current_time), device_stream){
            cur.push((prop_num,is_violated));
        }
    }
    result
}

pub fn eventually_prop_helper(operations: Vec<Operation>, bound: (i128,i128))-> Program {
    Program{ expressions: vec![], environment: Some(vec![OutputStream{
        ltl: LTL::Eventually(false),
        bound: Some(bound),
        time_verdicts: vec![],
        operations,
    }]) }
}

pub fn always_prop_helper(operations: Vec<Operation>, bound: Option<(i128,i128)>)-> Program {
    Program{ expressions: vec![], environment: Some(vec![OutputStream{
        ltl: LTL::Always,
        bound,
        time_verdicts: vec![],
        operations,
    }]) }
}

pub fn single_device_stream()->IoTStream{
    let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba".into(), 5, true).into(),
            ]
        ).into();
        temp_iot_stream
}

pub fn two_device_stream()->IoTStream{
    let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba".into(), 5, true).into(),
                ("christian".into(), 10, true).into(),
            ]
        ).into();
    temp_iot_stream
}

pub fn ten_device_stream()->IoTStream{
    let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba0".into(), 10, false).into(),
                ("Roomba1".into(), 20, true).into(),
                ("Roomba2".into(), 30, false).into(),
                ("Roomba3".into(), 40, true).into(),
                ("Roomba4".into(), 50, false).into(),
                ("Roomba5".into(), 60, true).into(),
                ("Roomba6".into(), 70, false).into(),
                ("Roomba7".into(), 80, true).into(),
                ("Roomba8".into(), 90, false).into(),
                ("Roomba9".into(), 100, true).into(),
            ]
        ).into();
    temp_iot_stream
}