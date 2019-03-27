/// Any type must implement this trait for serialization and deserialization
pub trait Desse<T>: Sized {
    /// Returns the size of serialized byte array
    fn serialized_size(&self) -> usize;

    /// Serializes current object
    fn serialize(&self) -> T;

    /// Deserializes an object from byte
    fn deserialize_from(bytes: &T) -> Self;
}

impl Desse<[u8; 4]> for [u16; 2] {
    fn serialized_size(&self) -> usize {
        4
    }

    fn serialize(&self) -> [u8; 4] {
        let mut bytes: [u8; 4] = [0; 4];
        (&mut bytes[0..2]).copy_from_slice(&self[0].serialize());
        (&mut bytes[2..4]).copy_from_slice(&self[1].serialize());
        bytes
    }

    fn deserialize_from(bytes: &[u8; 4]) -> Self {
        let mut arr: [u16; 2] = [0; 2];

        unsafe {
            // Cast &[u8] to &[u8; N]
            arr[0] = u16::deserialize_from(&*(bytes[0..2].as_ptr() as *const [u8; 2]));
            arr[1] = u16::deserialize_from(&*(bytes[2..4].as_ptr() as *const [u8; 2]));
        }

        arr
    }
}

macro_rules! impl_desse {
    ($type: ty) => {
        impl Desse<[u8; std::mem::size_of::<Self>()]> for $type {
            #[inline(always)]
            fn serialized_size(&self) -> usize {
                std::mem::size_of::<Self>()
            }

            #[inline(always)]
            fn serialize(&self) -> [u8; std::mem::size_of::<Self>()] {
                self.to_le_bytes()
            }

            #[inline(always)]
            fn deserialize_from(bytes: &[u8; std::mem::size_of::<Self>()]) -> Self {
                Self::from_le_bytes(*bytes)
            }
        }
    };
}

macro_rules! impl_desse_u8_arr {
    ($num: expr) => {
        impl Desse<[u8; $num]> for [u8; $num] {
            fn serialized_size(&self) -> usize {
                $num
            }

            fn serialize(&self) -> [u8; $num] {
                *self
            }

            fn deserialize_from(bytes: &[u8; $num]) -> Self {
                *bytes
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

impl_desse_u8_arr!(1);
impl_desse_u8_arr!(2);
impl_desse_u8_arr!(3);
impl_desse_u8_arr!(4);
impl_desse_u8_arr!(5);
impl_desse_u8_arr!(6);
impl_desse_u8_arr!(7);
impl_desse_u8_arr!(8);
impl_desse_u8_arr!(16);
impl_desse_u8_arr!(32);
impl_desse_u8_arr!(64);
impl_desse_u8_arr!(128);
impl_desse_u8_arr!(256);
impl_desse_u8_arr!(512);
impl_desse_u8_arr!(1024);
impl_desse_u8_arr!(2048);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_desse_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let num: $type = rand::random::<$type>();
                let new_num = <$type>::deserialize_from(&num.serialize());
                assert_eq!(num, new_num, "Invalid serialization / deserialization")
            }
        };
    }

    impl_desse_test!(u8, check_primitive_u8);
    impl_desse_test!(u16, check_primitive_u16);
    impl_desse_test!(u32, check_primitive_u32);
    impl_desse_test!(u64, check_primitive_u64);
    impl_desse_test!(u128, check_primitive_u128);

    impl_desse_test!(i8, check_primitive_i8);
    impl_desse_test!(i16, check_primitive_i16);
    impl_desse_test!(i32, check_primitive_i32);
    impl_desse_test!(i64, check_primitive_i64);
    impl_desse_test!(i128, check_primitive_i128);

    impl_desse_test!([u8; 1], check_arr_u8_1);
    impl_desse_test!([u8; 2], check_arr_u8_2);
    impl_desse_test!([u8; 3], check_arr_u8_3);
    impl_desse_test!([u8; 4], check_arr_u8_4);
    impl_desse_test!([u8; 5], check_arr_u8_5);
    impl_desse_test!([u8; 6], check_arr_u8_6);
    impl_desse_test!([u8; 7], check_arr_u8_7);
    impl_desse_test!([u8; 8], check_arr_u8_8);
    impl_desse_test!([u8; 16], check_arr_u8_16);
    impl_desse_test!([u8; 32], check_arr_u8_32);

    impl_desse_test!([u16; 2], check_arr_u16_2);
}
