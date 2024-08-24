use std::collections::HashMap;

pub mod errors;
pub mod types;
use types::Value;

#[trait_variant::make(IndustrialDevice: Send)]
pub trait LocalIndustrialDevice {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send>>;
    async fn dump_registers(
        &mut self,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error + Send>>;
}
