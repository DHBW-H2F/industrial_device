use std::collections::HashMap;

use async_trait::async_trait;

pub mod errors;
pub mod types;

use errors::IndustrialDeviceError;
use types::Value;

#[async_trait]
/// This code snippet defines a trait named `IndustrialDevice` in Rust. It is the "interface" of the device we need to collect the data
pub trait IndustrialDevice {
    async fn connect(&mut self) -> Result<(), IndustrialDeviceError>;
    async fn read_register_by_name(&mut self, name: &str) -> Result<Value, IndustrialDeviceError>;
    async fn write_register_by_name(
        &mut self,
        name: &str,
        value: &Value,
    ) -> Result<(), IndustrialDeviceError>;
    async fn dump_registers(&mut self) -> Result<HashMap<String, Value>, IndustrialDeviceError>;
}