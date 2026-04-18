pub mod streams;
pub mod types;
pub mod operation_eval;

#[cfg(test)]
mod streams_test;
#[cfg(test)]
mod operation_eval_test;

use std::error::Error;
use crate::{monitor::streams::{IoTStream, OutputStream}, program::Program};
use tokio::time::{Duration, interval};
use std::time::Instant;


use colored::Colorize;

impl Program {

    pub async fn monitor(&mut self, time_interval: i128, speed: bool) -> Result<(), Box<dyn Error>> {
        println!("Monitor has started...");
        
        let Some(streams) = &mut self.environment else { todo!() }; //Overvej custom error
        let mut interval = interval(Duration::from_millis(time_interval as u64));

        let mut t = 0;
        println!("{:#?}", streams);
        
        let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba".into(), 5, true).into(),
                //("christian".into(), 5, true).into(),
            ]
        ).into();

        loop {
            //#[cfg(not(debug_assertions))]
            if !speed{
                interval.tick().await;
            }

            let start = Instant::now();
            println!("--- Interval {:<4}", format!("{}",t).blue().bold());
            //todo: await devices
            async {
                for (prop_num, _) in Self::monitor_logic(streams, &t, &temp_iot_stream) {
                    let msg = format!("Prop {} violated", prop_num + 1);
                    println!("\t{} at time: {}", msg.red().bold().underline(), format!("{}s",t).on_bright_red().blue().bold());
                }
                t += time_interval / 1000;
            }.await;

            let elapsed = start.elapsed();
            let colored_time = if elapsed.as_millis() > time_interval as u128 { format!("{:?}",elapsed).red().bold() } 
                else { format!("{:?}",elapsed).bright_green().bold() };
            // println!("--- Interval {:<4} | Execution Time: {:>10} ---", format!("{}",t/1000).blue().bold(), colored_time);
            println!("\tExecution Time: {}", colored_time);

        }
    }

    
    fn monitor_logic<'a>(env: &'a mut [OutputStream], t: &'a i128, device_stream: &'a IoTStream) -> Box<dyn Iterator<Item = (usize, bool)> + 'a> {
        Box::new(
            env
                .iter_mut()
                .enumerate()
                .map(|(prop_num, output_stream)| {
                    let t = *t;
                    
                    // SDI update
                    output_stream.insert(t); 

                    // Calculate the new state of the streams
                    output_stream.update(t, device_stream); 

                    // Give verdicts
                    let is_violated = output_stream.get_violated_verdict_single(t);
                    
                    #[cfg(debug_assertions)]
                    println!("{:#?}", output_stream);
                    output_stream.clean_up();

                    (prop_num, is_violated)
                }).filter(|(_, v)| *v)
        )
    }
}
