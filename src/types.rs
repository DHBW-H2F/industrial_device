#[derive(Debug, Clone)]
/// This Rust code defines a public enumeration called `Value`. 'Value' represent different types of
/// values that the device can get from the data, including unsigned integers of 16, 32, 64, and 128 bits (`U16`, `U32`, `U64`, `U128`),
/// signed integers of 16 and 32 bits (`S16`, `S32`), an enum with a 16-bit unsigned integer (`Enum16`),
/// a fixed-size array of 66 bytes (`Sized`), a 32-bit floating-point number (`Float32`), and a boolean
/// value (`Boolean`).
pub enum Value {
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    S16(i16),
    S32(i32),

    Enum16(u16),

    Sized([u8; 66]),

    Float32(f32),

    Boolean(bool),
}
