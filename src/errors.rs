use custom_error::custom_error;


custom_error! {
    /// This code snippet in Rust is using a macro called `custom_error!` to define a custom error type
    /// called `IndustrialDeviceError`. This custom error type can represent various error scenarios
    /// specific to industrial devices.
    pub IndustrialDeviceError
    DeviceNotAccessibleError{err: Box<dyn std::error::Error + Send>} = "The device could not be reached : {err:?}",
    RequestError{err: Box<dyn std::error::Error + Send>} = "There was an error during the request : {err:?}",
    DeviceNotConnectedError{err: Box<dyn std::error::Error + Send>} = "The device is not connected, you have to ::connect before making requests : {err:?}",
    ConversionError{err: Box<dyn std::error::Error + Send>} = "There was an error converting data : {err:?}",
    RegisterNotFoundError{ name: String } = "The register could not be found : {name:?}",
    WrongValueType{ val: String } = "The Value \"{val}\" can not be converted for this device"
}
