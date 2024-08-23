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
