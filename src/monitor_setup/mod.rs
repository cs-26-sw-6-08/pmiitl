mod expression_rules;
mod property_rules;
pub mod streams;
mod types;

use crate::monitor_setup::streams::Streams;
use std::{error::Error};
use crate::{program::Program, errors};

impl Program {
    pub fn setup_streams(&mut self) -> Result<(), Box<dyn Error>> {
        // self.environment = Some(Streams::new()?);
        Ok(())
    }
}