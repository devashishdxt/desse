/// Any type must implement this trait for serialization and deserialization
pub trait Desse {
    /// Type of output
    type Output;

    /// Serializes current object
    fn serialize(&self) -> Self::Output;

    /// Deserializes an object
    fn deserialize_from(bytes: &Self::Output) -> Self;
}

macro_rules! impl_desse {
    ($type: ty) => {
        impl Desse for $type {
            type Output = [u8; std::mem::size_of::<Self>()];

            #[inline(always)]
            fn serialize(&self) -> Self::Output {
                self.to_le_bytes()
            }

            #[inline(always)]
            fn deserialize_from(bytes: &Self::Output) -> Self {
                Self::from_le_bytes(*bytes)
            }
        }
    };
}

macro_rules! impl_desse_arr {
    ([$type: ty; $num: expr]) => {
        impl Desse for [$type; $num] {
            type Output = [u8; std::mem::size_of::<$type>() * $num];

            #[inline(always)]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = Default::default();

                let mut counter = 0;

                for element in self {
                    (&mut bytes[counter..(counter + std::mem::size_of::<$type>())])
                        .copy_from_slice(&element.serialize());
                    counter += std::mem::size_of::<$type>();
                }

                bytes
            }

            #[inline(always)]
            fn deserialize_from(bytes: &Self::Output) -> Self {
                let mut arr: Self = Default::default();

                let mut counter = 0;

                for i in 0..$num {
                    unsafe {
                        arr[i] = <$type>::deserialize_from(
                            &*(bytes[counter..(counter + std::mem::size_of::<$type>())].as_ptr()
                                as *const [u8; std::mem::size_of::<$type>()]),
                        );
                    }

                    counter += std::mem::size_of::<$type>();
                }

                arr
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

impl_desse_arr!([u8; 1]);
impl_desse_arr!([u8; 2]);
impl_desse_arr!([u8; 3]);
impl_desse_arr!([u8; 4]);
impl_desse_arr!([u8; 5]);
impl_desse_arr!([u8; 6]);
impl_desse_arr!([u8; 7]);
impl_desse_arr!([u8; 8]);
impl_desse_arr!([u8; 16]);
impl_desse_arr!([u8; 32]);

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
}
