use byteorder::{ByteOrder, LittleEndian};

/// Any type must implement this trait for serialization and deserialization
pub trait Desse<T>: Sized {
    /// Serializes current object
    fn serialize(&self) -> T;

    /// Deserializes an object from byte
    fn deserialize_from(bytes: &T) -> Self;
}

impl Desse<[u8; 1]> for u8 {
    fn serialize(&self) -> [u8; 1] {
        [*self]
    }

    fn deserialize_from(bytes: &[u8; 1]) -> Self {
        bytes[0]
    }
}

impl Desse<[u8; 2]> for u16 {
    fn serialize(&self) -> [u8; 2] {
        let mut bytes: [u8; 2] = Default::default();
        LittleEndian::write_u16(&mut bytes, *self);

        bytes
    }

    fn deserialize_from(bytes: &[u8; 2]) -> Self {
        LittleEndian::read_u16(bytes)
    }
}

impl Desse<[u8; 4]> for u32 {
    fn serialize(&self) -> [u8; 4] {
        let mut bytes: [u8; 4] = Default::default();
        LittleEndian::write_u32(&mut bytes, *self);

        bytes
    }

    fn deserialize_from(bytes: &[u8; 4]) -> Self {
        LittleEndian::read_u32(bytes)
    }
}

impl Desse<[u8; 8]> for u64 {
    fn serialize(&self) -> [u8; 8] {
        let mut bytes: [u8; 8] = Default::default();
        LittleEndian::write_u64(&mut bytes, *self);

        bytes
    }

    fn deserialize_from(bytes: &[u8; 8]) -> Self {
        LittleEndian::read_u64(bytes)
    }
}

impl Desse<[u8; 16]> for u128 {
    fn serialize(&self) -> [u8; 16] {
        let mut bytes: [u8; 16] = Default::default();
        LittleEndian::write_u128(&mut bytes, *self);

        bytes
    }

    fn deserialize_from(bytes: &[u8; 16]) -> Self {
        LittleEndian::read_u128(bytes)
    }
}
