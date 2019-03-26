/// Any type must implement this trait for serialization and deserialization
pub trait Desse: Sized {
    /// Returns the size of byte array returned on serialization
    fn serialize_size(&self) -> u64;

    /// Serializes current object
    fn serialize(&self) -> Vec<u8>;

    /// Deserializes an object from byte
    fn deserialize_from(bytes: &[u8]) -> Self;
}
