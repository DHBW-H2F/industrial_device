use custom_error::custom_error;

custom_error! {pub IndustrialDeviceError
    DeviceNotAccessibleError{err: Box<dyn std::error::Error + Send>} = "The device could not be reached : {err:?}",
    RequestError{err: Box<dyn std::error::Error + Send>} = "There was an error during the request : {err:?}",
    DeviceNotConnectedError{err: Box<dyn std::error::Error + Send>} = "The device is not connected, you have to ::connect before making requests : {err:?}",
    ConversionError{err: Box<dyn std::error::Error + Send>} = "There was an error converting data : {err:?}",
    RegisterNotFoundError{err: Box<dyn std::error::Error + Send>} = "The register could not be found : {err:?}",
}
