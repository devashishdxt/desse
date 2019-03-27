/// Any type must implement this trait for serialization and deserialization
pub trait Desse<T>: Sized {
    /// Serializes current object
    fn serialize(&self) -> T;

    /// Deserializes an object from byte
    fn deserialize_from(bytes: &T) -> Self;
}

macro_rules! impl_desse {
    ($type: ty) => {
        impl Desse<[u8; std::mem::size_of::<Self>()]> for $type {
            fn serialize(&self) -> [u8; std::mem::size_of::<Self>()] {
                self.to_le_bytes()
            }

            fn deserialize_from(bytes: &[u8; std::mem::size_of::<Self>()]) -> Self {
                Self::from_le_bytes(*bytes)
            }
        }
    };
}

impl_desse!(u8);
impl_desse!(u16);
impl_desse!(u32);
impl_desse!(u64);
impl_desse!(u128);

impl_desse!(i8);
impl_desse!(i16);
impl_desse!(i32);
impl_desse!(i64);
impl_desse!(i128);
