use std::collections::HashMap;

mod types;
use types::Value;

#[trait_variant::make(IndustrialDevice: Send)]
pub trait LocalIndustrialDevice {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send>>;
    async fn dump_registers(
        &mut self,
    ) -> Result<HashMap<String, Value>, impl std::error::Error + Send>;
}
