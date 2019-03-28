/// Any type must implement this trait for serialization and deserialization
pub trait Desse: DesseSized {
    /// Type of output
    type Output;

    /// Serializes current object
    fn serialize(&self) -> Self::Output;

    /// Deserializes an object
    fn deserialize_from(bytes: &Self::Output) -> Self;
}

/// Any trait must implement this to implement [`Desse`]
pub trait DesseSized: Sized {
    /// Size of output byte array
    const SIZE: usize;
}

macro_rules! impl_desse {
    ($type: ty) => {
        impl DesseSized for $type {
            const SIZE: usize = std::mem::size_of::<Self>();
        }

        impl Desse for $type {
            type Output = [u8; Self::SIZE];

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

macro_rules! impl_generic_desse_size_arr {
    ($num: expr) => {
        impl<T> DesseSized for [T; $num]
        where
            T: DesseSized,
        {
            const SIZE: usize = <T>::SIZE * $num;
        }
    };
}

macro_rules! impl_desse_arr {
    ([$type: ty; $num: expr]) => {
        impl Desse for [$type; $num] {
            type Output = [u8; Self::SIZE];

            #[inline(always)]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = [0; Self::SIZE];

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

impl_generic_desse_size_arr!(1);
impl_generic_desse_size_arr!(2);
impl_generic_desse_size_arr!(3);
impl_generic_desse_size_arr!(4);
impl_generic_desse_size_arr!(5);
impl_generic_desse_size_arr!(6);
impl_generic_desse_size_arr!(7);
impl_generic_desse_size_arr!(8);
impl_generic_desse_size_arr!(16);
impl_generic_desse_size_arr!(32);

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

impl_desse_arr!([u16; 1]);
impl_desse_arr!([u16; 2]);
impl_desse_arr!([u16; 3]);
impl_desse_arr!([u16; 4]);
impl_desse_arr!([u16; 5]);
impl_desse_arr!([u16; 6]);
impl_desse_arr!([u16; 7]);
impl_desse_arr!([u16; 8]);
impl_desse_arr!([u16; 16]);
impl_desse_arr!([u16; 32]);

impl_desse_arr!([u32; 1]);
impl_desse_arr!([u32; 2]);
impl_desse_arr!([u32; 3]);
impl_desse_arr!([u32; 4]);
impl_desse_arr!([u32; 5]);
impl_desse_arr!([u32; 6]);
impl_desse_arr!([u32; 7]);
impl_desse_arr!([u32; 8]);
impl_desse_arr!([u32; 16]);
impl_desse_arr!([u32; 32]);

impl_desse_arr!([u64; 1]);
impl_desse_arr!([u64; 2]);
impl_desse_arr!([u64; 3]);
impl_desse_arr!([u64; 4]);
impl_desse_arr!([u64; 5]);
impl_desse_arr!([u64; 6]);
impl_desse_arr!([u64; 7]);
impl_desse_arr!([u64; 8]);
impl_desse_arr!([u64; 16]);
impl_desse_arr!([u64; 32]);

impl_desse_arr!([u128; 1]);
impl_desse_arr!([u128; 2]);
impl_desse_arr!([u128; 3]);
impl_desse_arr!([u128; 4]);
impl_desse_arr!([u128; 5]);
impl_desse_arr!([u128; 6]);
impl_desse_arr!([u128; 7]);
impl_desse_arr!([u128; 8]);
impl_desse_arr!([u128; 16]);
impl_desse_arr!([u128; 32]);

impl_desse_arr!([i8; 1]);
impl_desse_arr!([i8; 2]);
impl_desse_arr!([i8; 3]);
impl_desse_arr!([i8; 4]);
impl_desse_arr!([i8; 5]);
impl_desse_arr!([i8; 6]);
impl_desse_arr!([i8; 7]);
impl_desse_arr!([i8; 8]);
impl_desse_arr!([i8; 16]);
impl_desse_arr!([i8; 32]);

impl_desse_arr!([i16; 1]);
impl_desse_arr!([i16; 2]);
impl_desse_arr!([i16; 3]);
impl_desse_arr!([i16; 4]);
impl_desse_arr!([i16; 5]);
impl_desse_arr!([i16; 6]);
impl_desse_arr!([i16; 7]);
impl_desse_arr!([i16; 8]);
impl_desse_arr!([i16; 16]);
impl_desse_arr!([i16; 32]);

impl_desse_arr!([i32; 1]);
impl_desse_arr!([i32; 2]);
impl_desse_arr!([i32; 3]);
impl_desse_arr!([i32; 4]);
impl_desse_arr!([i32; 5]);
impl_desse_arr!([i32; 6]);
impl_desse_arr!([i32; 7]);
impl_desse_arr!([i32; 8]);
impl_desse_arr!([i32; 16]);
impl_desse_arr!([i32; 32]);

impl_desse_arr!([i64; 1]);
impl_desse_arr!([i64; 2]);
impl_desse_arr!([i64; 3]);
impl_desse_arr!([i64; 4]);
impl_desse_arr!([i64; 5]);
impl_desse_arr!([i64; 6]);
impl_desse_arr!([i64; 7]);
impl_desse_arr!([i64; 8]);
impl_desse_arr!([i64; 16]);
impl_desse_arr!([i64; 32]);

impl_desse_arr!([i128; 1]);
impl_desse_arr!([i128; 2]);
impl_desse_arr!([i128; 3]);
impl_desse_arr!([i128; 4]);
impl_desse_arr!([i128; 5]);
impl_desse_arr!([i128; 6]);
impl_desse_arr!([i128; 7]);
impl_desse_arr!([i128; 8]);
impl_desse_arr!([i128; 16]);
impl_desse_arr!([i128; 32]);

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

    impl_desse_test!([u16; 1], check_arr_u16_1);
    impl_desse_test!([u16; 2], check_arr_u16_2);
    impl_desse_test!([u16; 3], check_arr_u16_3);
    impl_desse_test!([u16; 4], check_arr_u16_4);
    impl_desse_test!([u16; 5], check_arr_u16_5);
    impl_desse_test!([u16; 6], check_arr_u16_6);
    impl_desse_test!([u16; 7], check_arr_u16_7);
    impl_desse_test!([u16; 8], check_arr_u16_8);
    impl_desse_test!([u16; 16], check_arr_u16_16);
    impl_desse_test!([u16; 32], check_arr_u16_32);

    impl_desse_test!([u32; 1], check_arr_u32_1);
    impl_desse_test!([u32; 2], check_arr_u32_2);
    impl_desse_test!([u32; 3], check_arr_u32_3);
    impl_desse_test!([u32; 4], check_arr_u32_4);
    impl_desse_test!([u32; 5], check_arr_u32_5);
    impl_desse_test!([u32; 6], check_arr_u32_6);
    impl_desse_test!([u32; 7], check_arr_u32_7);
    impl_desse_test!([u32; 8], check_arr_u32_8);
    impl_desse_test!([u32; 16], check_arr_u32_16);
    impl_desse_test!([u32; 32], check_arr_u32_32);

    impl_desse_test!([u64; 1], check_arr_u64_1);
    impl_desse_test!([u64; 2], check_arr_u64_2);
    impl_desse_test!([u64; 3], check_arr_u64_3);
    impl_desse_test!([u64; 4], check_arr_u64_4);
    impl_desse_test!([u64; 5], check_arr_u64_5);
    impl_desse_test!([u64; 6], check_arr_u64_6);
    impl_desse_test!([u64; 7], check_arr_u64_7);
    impl_desse_test!([u64; 8], check_arr_u64_8);
    impl_desse_test!([u64; 16], check_arr_u64_16);
    impl_desse_test!([u64; 32], check_arr_u64_32);

    impl_desse_test!([u128; 1], check_arr_u128_1);
    impl_desse_test!([u128; 2], check_arr_u128_2);
    impl_desse_test!([u128; 3], check_arr_u128_3);
    impl_desse_test!([u128; 4], check_arr_u128_4);
    impl_desse_test!([u128; 5], check_arr_u128_5);
    impl_desse_test!([u128; 6], check_arr_u128_6);
    impl_desse_test!([u128; 7], check_arr_u128_7);
    impl_desse_test!([u128; 8], check_arr_u128_8);
    impl_desse_test!([u128; 16], check_arr_u128_16);
    impl_desse_test!([u128; 32], check_arr_u128_32);

    impl_desse_test!([i8; 1], check_arr_i8_1);
    impl_desse_test!([i8; 2], check_arr_i8_2);
    impl_desse_test!([i8; 3], check_arr_i8_3);
    impl_desse_test!([i8; 4], check_arr_i8_4);
    impl_desse_test!([i8; 5], check_arr_i8_5);
    impl_desse_test!([i8; 6], check_arr_i8_6);
    impl_desse_test!([i8; 7], check_arr_i8_7);
    impl_desse_test!([i8; 8], check_arr_i8_8);
    impl_desse_test!([i8; 16], check_arr_i8_16);
    impl_desse_test!([i8; 32], check_arr_i8_32);

    impl_desse_test!([i16; 1], check_arr_i16_1);
    impl_desse_test!([i16; 2], check_arr_i16_2);
    impl_desse_test!([i16; 3], check_arr_i16_3);
    impl_desse_test!([i16; 4], check_arr_i16_4);
    impl_desse_test!([i16; 5], check_arr_i16_5);
    impl_desse_test!([i16; 6], check_arr_i16_6);
    impl_desse_test!([i16; 7], check_arr_i16_7);
    impl_desse_test!([i16; 8], check_arr_i16_8);
    impl_desse_test!([i16; 16], check_arr_i16_16);
    impl_desse_test!([i16; 32], check_arr_i16_32);

    impl_desse_test!([i32; 1], check_arr_i32_1);
    impl_desse_test!([i32; 2], check_arr_i32_2);
    impl_desse_test!([i32; 3], check_arr_i32_3);
    impl_desse_test!([i32; 4], check_arr_i32_4);
    impl_desse_test!([i32; 5], check_arr_i32_5);
    impl_desse_test!([i32; 6], check_arr_i32_6);
    impl_desse_test!([i32; 7], check_arr_i32_7);
    impl_desse_test!([i32; 8], check_arr_i32_8);
    impl_desse_test!([i32; 16], check_arr_i32_16);
    impl_desse_test!([i32; 32], check_arr_i32_32);

    impl_desse_test!([i64; 1], check_arr_i64_1);
    impl_desse_test!([i64; 2], check_arr_i64_2);
    impl_desse_test!([i64; 3], check_arr_i64_3);
    impl_desse_test!([i64; 4], check_arr_i64_4);
    impl_desse_test!([i64; 5], check_arr_i64_5);
    impl_desse_test!([i64; 6], check_arr_i64_6);
    impl_desse_test!([i64; 7], check_arr_i64_7);
    impl_desse_test!([i64; 8], check_arr_i64_8);
    impl_desse_test!([i64; 16], check_arr_i64_16);
    impl_desse_test!([i64; 32], check_arr_i64_32);

    impl_desse_test!([i128; 1], check_arr_i128_1);
    impl_desse_test!([i128; 2], check_arr_i128_2);
    impl_desse_test!([i128; 3], check_arr_i128_3);
    impl_desse_test!([i128; 4], check_arr_i128_4);
    impl_desse_test!([i128; 5], check_arr_i128_5);
    impl_desse_test!([i128; 6], check_arr_i128_6);
    impl_desse_test!([i128; 7], check_arr_i128_7);
    impl_desse_test!([i128; 8], check_arr_i128_8);
    impl_desse_test!([i128; 16], check_arr_i128_16);
    impl_desse_test!([i128; 32], check_arr_i128_32);
}
