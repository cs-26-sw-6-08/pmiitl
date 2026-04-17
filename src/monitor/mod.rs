pub mod streams;
pub mod types;
pub mod operation_eval;

#[cfg(test)]
mod streams_test;

use std::error::Error;
use crate::{monitor::streams::{IoTDevice, IoTStream}, monitor_setup::operation_types::LTL, program::Program};
use tokio::time::{Duration, interval};
use std::time::Instant;


use colored::*;

impl Program {

    pub async fn monitor(&mut self, time_interval: i128) -> Result<(), Box<dyn Error>> {
        println!("Monitor has started...");
        
        let Some(streams) = &mut self.environment else { todo!() }; //Overvej custom error
        let mut interval = interval(Duration::from_millis(time_interval as u64));

        let mut t = 0;
        
        let temp_iot_stream = IoTStream::from(vec![IoTDevice::from((String::from("Roomba"), 5, true))]);

        loop {
            interval.tick().await;
            let start = Instant::now();
            
            //todo: await devices

            async {

                let violated_verdicts = streams
                .iter_mut()
                .enumerate()
                .map(|(prop_num, output_stream)| {
                    if output_stream.gone {
                        return (prop_num, false)
                    }
                    // SDI update
                    output_stream.insert(t); 

                    // Calculate the new state of the streams
                    output_stream.update(t, &temp_iot_stream); 

                    // Give verdicts
                    let is_violated; 
                    match &mut output_stream.ltl {
                        LTL::Always => is_violated = output_stream.get_violated_verdict_single(t),
                        LTL::Eventually => {
                            is_violated = output_stream.get_violated_verdict_single(t);
                            if is_violated {
                                println!("{}", format!("--- Removed {prop_num} ---").yellow().bold().italic().underline());
                                output_stream.gone = true;
                            }
                        },
                    }
                    
                    println!("is_violated: {is_violated}");
                    if cfg!(debug_assertions) {
                        println!("{:#?}", output_stream);
                    }
                    output_stream.clean_up();


                    (prop_num, is_violated)
                }).filter(|(_, v)| *v);

                for (prop_num, _) in violated_verdicts {
                    println!("\t{} at time: {t}", format!("Prop {} violated", prop_num+1).red());
                }

                
                t += time_interval;
            }.await;

            let elapsed = start.elapsed();
            let colored_time = if elapsed.as_millis() > time_interval as u128 { format!("{:?}",elapsed).red() } 
                else { format!("{:?}",elapsed).green() };
            // println!("Yarjis translate to engliesh det tog så lang tid at regne for {} her er tiden {}", t, colored_time);
            println!("--- Interval {:<4} | Execution Time: {:>10} ---", format!("{}",t/1000).bright_cyan().italic().underline(), colored_time);
        }
    }
}