pub mod streams;
pub mod types;
pub mod operation_eval;

#[cfg(test)]
mod streams_test;
#[cfg(test)]
mod operation_eval_test;

use std::error::Error;
use crate::{errors, monitor::streams::{IoTStream, OutputStream}, program::Program};
use tokio::time::{Duration, interval};
use std::time::Instant;


use colored::Colorize;

impl Program {
    pub async fn monitor(&mut self, time_interval: i128, speed: bool) -> Result<(), Box<dyn Error>> {
        println!("Monitor has started...");
        
        let Some(streams) = &mut self.environment else { return Err(errors::Error::EnvironmentNotPresent.into()); };
        let mut interval = interval(Duration::from_millis(time_interval as u64));

        let mut t = 0;
        #[cfg(debug_assertions)]
        println!("{}",format!("{:#?}", streams).red());
        
        let temp_iot_stream: IoTStream = (
            vec![
                ("Roomba".into(), 5).into(),
                ("Roomba".into(), 5).into(),
                ("Roomba".into(), 5).into(),
                ("Roomba".into(), 5).into(),
                ("Roomba".into(), 5).into(),
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
                for el in Self::monitor_logic(streams, &t, &temp_iot_stream) {
                    let (prop_num, _ )=  el?; 
                    let msg = format!("Prop {} violated", prop_num + 1);
                    println!("\t{} at time: {}", msg.red().bold().underline(), format!("{}s",t).on_bright_red().blue().bold());
                }
                t += time_interval / 1000;
                
                Ok::<(), Box<dyn Error>>(())
            }.await?;

            let elapsed = start.elapsed();
            let colored_time = if elapsed.as_millis() > time_interval as u128 { format!("{:?}",elapsed).red().bold() } 
                else { format!("{:?}",elapsed).bright_green().bold() };
            // println!("--- Interval {:<4} | Execution Time: {:>10} ---", format!("{}",t/1000).blue().bold(), colored_time);
            println!("\tExecution Time: {}", colored_time);

        }
    }

    
    pub fn monitor_logic<'a>(env: &'a mut [OutputStream], t: &'a i128, device_stream: &'a IoTStream) -> Box<dyn Iterator<Item = Result<(usize, bool), Box<dyn Error>>> + 'a> {
        Box::new(
            env
                .iter_mut()
                .enumerate()
                .map(|(prop_num, output_stream)| {
                    let t = *t;
                    
                    // SDI update
                    output_stream.insert(t); 

                    // Calculate the new state of the streams
                    //todo: potentially add error handling
                    let _ = output_stream.update(t, device_stream); 
                    #[cfg(debug_assertions)]
                    println!("{:#?}", output_stream);

                    // Give verdicts
                    let is_violated = output_stream.get_violated_verdict_single();
                    
                    output_stream.clean_up();

                    Ok((prop_num, is_violated))
                }).filter(|el| el.as_ref().map(|(_, v)| *v).unwrap_or(true))
        )
    }
}
