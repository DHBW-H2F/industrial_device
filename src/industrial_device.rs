use std::collections::HashMap;

pub mod errors;
pub mod types;

use errors::IndustrialDeviceError;
use types::Value;

#[trait_variant::make(IndustrialDevice: Send)]
pub trait LocalIndustrialDevice {
    async fn connect(&mut self) -> Result<(), IndustrialDeviceError>;
    async fn read_register_by_name(&mut self, name: &str) -> Result<Value, IndustrialDeviceError>;
    async fn write_register_by_name(
        &mut self,
        name: &str,
        value: &Value,
    ) -> Result<(), IndustrialDeviceError>;
    async fn dump_registers(&mut self) -> Result<HashMap<String, Value>, IndustrialDeviceError>;
}
