use crate::{ErrorKind, Result};

/// Any type must implement this trait for serialization and deserialization
pub trait Desse: Sized {
    /// Type of output
    type Output;

    /// Serializes current object
    fn serialize(&self) -> Self::Output;

    /// Serializes current object into bytes
    fn serialize_into(&self, bytes: &mut Self::Output);

    /// Deserializes an object
    fn deserialize_from(bytes: &Self::Output) -> Result<Self>;
}

/// Helper trait used to compute `SIZE` of a type at compile time
pub trait DesseSized {
    /// Size of output byte array
    const SIZE: usize;
}

macro_rules! impl_desse {
    ($type: ty) => {
        impl DesseSized for $type {
            const SIZE: usize = core::mem::size_of::<Self>();
        }

        impl Desse for $type {
            type Output = [u8; Self::SIZE];

            #[inline]
            fn serialize(&self) -> Self::Output {
                self.to_le_bytes()
            }

            #[inline]
            fn serialize_into(&self, bytes: &mut Self::Output) {
                bytes.copy_from_slice(&self.serialize());
            }

            #[inline]
            fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
                Ok(Self::from_le_bytes(*bytes))
            }
        }
    };
}

macro_rules! impl_desse_size_generic_arr {
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

            #[inline]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = [0; Self::SIZE];
                self.serialize_into(&mut bytes);
                bytes
            }

            #[inline]
            fn serialize_into(&self, bytes: &mut Self::Output) {
                let mut counter = 0;

                for element in self {
                    unsafe {
                        Desse::serialize_into(
                            element,
                            &mut *(bytes[counter..(counter + <$type>::SIZE)].as_mut_ptr()
                                as *mut [u8; <$type>::SIZE]),
                        );
                    }
                    counter += <$type>::SIZE;
                }
            }

            #[inline]
            fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
                let mut arr: Self = [0; $num];

                let mut counter = 0;

                for i in 0..$num {
                    unsafe {
                        arr[i] = <$type>::deserialize_from(
                            &*(bytes[counter..(counter + <$type>::SIZE)].as_ptr()
                                as *const [u8; <$type>::SIZE]),
                        )?;
                    }

                    counter += <$type>::SIZE;
                }

                Ok(arr)
            }
        }
    };
}

macro_rules! impl_desse_arr_char {
    ($num: expr) => {
        impl Desse for [char; $num] {
            type Output = [u8; Self::SIZE];

            #[inline]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = [0; Self::SIZE];
                self.serialize_into(&mut bytes);
                bytes
            }

            #[inline]
            fn serialize_into(&self, bytes: &mut Self::Output) {
                let mut counter = 0;

                for element in self {
                    unsafe {
                        Desse::serialize_into(
                            element,
                            &mut *(bytes[counter..(counter + <char>::SIZE)].as_mut_ptr()
                                as *mut [u8; <char>::SIZE]),
                        );
                    }
                    counter += <char>::SIZE;
                }
            }

            #[inline]
            fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
                let mut arr: Self = [0 as char; $num];

                let mut counter = 0;

                for i in 0..$num {
                    unsafe {
                        arr[i] = <char>::deserialize_from(
                            &*(bytes[counter..(counter + <char>::SIZE)].as_ptr()
                                as *const [u8; <char>::SIZE]),
                        )?;
                    }

                    counter += <char>::SIZE;
                }

                Ok(arr)
            }
        }
    };
}

macro_rules! impl_desse_arr_bool {
    ($num: expr) => {
        impl Desse for [bool; $num] {
            type Output = [u8; Self::SIZE];

            #[inline]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = [0; Self::SIZE];
                self.serialize_into(&mut bytes);
                bytes
            }

            #[inline]
            fn serialize_into(&self, bytes: &mut Self::Output) {
                let mut counter = 0;

                for element in self {
                    unsafe {
                        Desse::serialize_into(
                            element,
                            &mut *(bytes[counter..(counter + <bool>::SIZE)].as_mut_ptr()
                                as *mut [u8; <bool>::SIZE]),
                        );
                    }
                    counter += <bool>::SIZE;
                }
            }

            #[inline]
            fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
                let mut arr: Self = [false; $num];

                let mut counter = 0;

                for i in 0..$num {
                    unsafe {
                        arr[i] = <bool>::deserialize_from(
                            &*(bytes[counter..(counter + <bool>::SIZE)].as_ptr()
                                as *const [u8; <bool>::SIZE]),
                        )?;
                    }

                    counter += <bool>::SIZE;
                }

                Ok(arr)
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

impl DesseSized for bool {
    const SIZE: usize = core::mem::size_of::<Self>();
}

impl Desse for bool {
    type Output = [u8; Self::SIZE];

    #[inline]
    fn serialize(&self) -> Self::Output {
        (*self as u8).to_le_bytes()
    }

    #[inline]
    fn serialize_into(&self, bytes: &mut Self::Output) {
        bytes.copy_from_slice(&self.serialize());
    }

    #[inline]
    fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
        Ok(u8::from_le_bytes(*bytes) != 0)
    }
}

impl DesseSized for char {
    const SIZE: usize = core::mem::size_of::<Self>();
}

impl Desse for char {
    type Output = [u8; Self::SIZE];

    #[inline]
    fn serialize(&self) -> Self::Output {
        (*self as u32).to_le_bytes()
    }

    #[inline]
    fn serialize_into(&self, bytes: &mut Self::Output) {
        bytes.copy_from_slice(&self.serialize());
    }

    #[inline]
    fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
        match core::char::from_u32(u32::from_le_bytes(*bytes)) {
            None => Err(ErrorKind::InvalidChar.into()),
            Some(c) => Ok(c),
        }
    }
}

impl_desse_size_generic_arr!(1);
impl_desse_size_generic_arr!(2);
impl_desse_size_generic_arr!(3);
impl_desse_size_generic_arr!(4);
impl_desse_size_generic_arr!(5);
impl_desse_size_generic_arr!(6);
impl_desse_size_generic_arr!(7);
impl_desse_size_generic_arr!(8);
impl_desse_size_generic_arr!(9);
impl_desse_size_generic_arr!(10);
impl_desse_size_generic_arr!(11);
impl_desse_size_generic_arr!(12);
impl_desse_size_generic_arr!(13);
impl_desse_size_generic_arr!(14);
impl_desse_size_generic_arr!(15);
impl_desse_size_generic_arr!(16);
impl_desse_size_generic_arr!(17);
impl_desse_size_generic_arr!(18);
impl_desse_size_generic_arr!(19);
impl_desse_size_generic_arr!(20);
impl_desse_size_generic_arr!(21);
impl_desse_size_generic_arr!(22);
impl_desse_size_generic_arr!(23);
impl_desse_size_generic_arr!(24);
impl_desse_size_generic_arr!(25);
impl_desse_size_generic_arr!(26);
impl_desse_size_generic_arr!(27);
impl_desse_size_generic_arr!(28);
impl_desse_size_generic_arr!(29);
impl_desse_size_generic_arr!(30);
impl_desse_size_generic_arr!(31);
impl_desse_size_generic_arr!(32);

impl_desse_arr_bool!(1);
impl_desse_arr_bool!(2);
impl_desse_arr_bool!(3);
impl_desse_arr_bool!(4);
impl_desse_arr_bool!(5);
impl_desse_arr_bool!(6);
impl_desse_arr_bool!(7);
impl_desse_arr_bool!(8);
impl_desse_arr_bool!(9);
impl_desse_arr_bool!(10);
impl_desse_arr_bool!(11);
impl_desse_arr_bool!(12);
impl_desse_arr_bool!(13);
impl_desse_arr_bool!(14);
impl_desse_arr_bool!(15);
impl_desse_arr_bool!(16);
impl_desse_arr_bool!(17);
impl_desse_arr_bool!(18);
impl_desse_arr_bool!(19);
impl_desse_arr_bool!(20);
impl_desse_arr_bool!(21);
impl_desse_arr_bool!(22);
impl_desse_arr_bool!(23);
impl_desse_arr_bool!(24);
impl_desse_arr_bool!(25);
impl_desse_arr_bool!(26);
impl_desse_arr_bool!(27);
impl_desse_arr_bool!(28);
impl_desse_arr_bool!(29);
impl_desse_arr_bool!(30);
impl_desse_arr_bool!(31);
impl_desse_arr_bool!(32);

impl_desse_arr_char!(1);
impl_desse_arr_char!(2);
impl_desse_arr_char!(3);
impl_desse_arr_char!(4);
impl_desse_arr_char!(5);
impl_desse_arr_char!(6);
impl_desse_arr_char!(7);
impl_desse_arr_char!(8);
impl_desse_arr_char!(9);
impl_desse_arr_char!(10);
impl_desse_arr_char!(11);
impl_desse_arr_char!(12);
impl_desse_arr_char!(13);
impl_desse_arr_char!(14);
impl_desse_arr_char!(15);
impl_desse_arr_char!(16);
impl_desse_arr_char!(17);
impl_desse_arr_char!(18);
impl_desse_arr_char!(19);
impl_desse_arr_char!(20);
impl_desse_arr_char!(21);
impl_desse_arr_char!(22);
impl_desse_arr_char!(23);
impl_desse_arr_char!(24);
impl_desse_arr_char!(25);
impl_desse_arr_char!(26);
impl_desse_arr_char!(27);
impl_desse_arr_char!(28);
impl_desse_arr_char!(29);
impl_desse_arr_char!(30);
impl_desse_arr_char!(31);
impl_desse_arr_char!(32);

impl_desse_arr!([u8; 1]);
impl_desse_arr!([u8; 2]);
impl_desse_arr!([u8; 3]);
impl_desse_arr!([u8; 4]);
impl_desse_arr!([u8; 5]);
impl_desse_arr!([u8; 6]);
impl_desse_arr!([u8; 7]);
impl_desse_arr!([u8; 8]);
impl_desse_arr!([u8; 9]);
impl_desse_arr!([u8; 10]);
impl_desse_arr!([u8; 11]);
impl_desse_arr!([u8; 12]);
impl_desse_arr!([u8; 13]);
impl_desse_arr!([u8; 14]);
impl_desse_arr!([u8; 15]);
impl_desse_arr!([u8; 16]);
impl_desse_arr!([u8; 17]);
impl_desse_arr!([u8; 18]);
impl_desse_arr!([u8; 19]);
impl_desse_arr!([u8; 20]);
impl_desse_arr!([u8; 21]);
impl_desse_arr!([u8; 22]);
impl_desse_arr!([u8; 23]);
impl_desse_arr!([u8; 24]);
impl_desse_arr!([u8; 25]);
impl_desse_arr!([u8; 26]);
impl_desse_arr!([u8; 27]);
impl_desse_arr!([u8; 28]);
impl_desse_arr!([u8; 29]);
impl_desse_arr!([u8; 30]);
impl_desse_arr!([u8; 31]);
impl_desse_arr!([u8; 32]);

impl_desse_arr!([u16; 1]);
impl_desse_arr!([u16; 2]);
impl_desse_arr!([u16; 3]);
impl_desse_arr!([u16; 4]);
impl_desse_arr!([u16; 5]);
impl_desse_arr!([u16; 6]);
impl_desse_arr!([u16; 7]);
impl_desse_arr!([u16; 8]);
impl_desse_arr!([u16; 9]);
impl_desse_arr!([u16; 10]);
impl_desse_arr!([u16; 11]);
impl_desse_arr!([u16; 12]);
impl_desse_arr!([u16; 13]);
impl_desse_arr!([u16; 14]);
impl_desse_arr!([u16; 15]);
impl_desse_arr!([u16; 16]);
impl_desse_arr!([u16; 17]);
impl_desse_arr!([u16; 18]);
impl_desse_arr!([u16; 19]);
impl_desse_arr!([u16; 20]);
impl_desse_arr!([u16; 21]);
impl_desse_arr!([u16; 22]);
impl_desse_arr!([u16; 23]);
impl_desse_arr!([u16; 24]);
impl_desse_arr!([u16; 25]);
impl_desse_arr!([u16; 26]);
impl_desse_arr!([u16; 27]);
impl_desse_arr!([u16; 28]);
impl_desse_arr!([u16; 29]);
impl_desse_arr!([u16; 30]);
impl_desse_arr!([u16; 31]);
impl_desse_arr!([u16; 32]);

impl_desse_arr!([u32; 1]);
impl_desse_arr!([u32; 2]);
impl_desse_arr!([u32; 3]);
impl_desse_arr!([u32; 4]);
impl_desse_arr!([u32; 5]);
impl_desse_arr!([u32; 6]);
impl_desse_arr!([u32; 7]);
impl_desse_arr!([u32; 8]);
impl_desse_arr!([u32; 9]);
impl_desse_arr!([u32; 10]);
impl_desse_arr!([u32; 11]);
impl_desse_arr!([u32; 12]);
impl_desse_arr!([u32; 13]);
impl_desse_arr!([u32; 14]);
impl_desse_arr!([u32; 15]);
impl_desse_arr!([u32; 16]);
impl_desse_arr!([u32; 17]);
impl_desse_arr!([u32; 18]);
impl_desse_arr!([u32; 19]);
impl_desse_arr!([u32; 20]);
impl_desse_arr!([u32; 21]);
impl_desse_arr!([u32; 22]);
impl_desse_arr!([u32; 23]);
impl_desse_arr!([u32; 24]);
impl_desse_arr!([u32; 25]);
impl_desse_arr!([u32; 26]);
impl_desse_arr!([u32; 27]);
impl_desse_arr!([u32; 28]);
impl_desse_arr!([u32; 29]);
impl_desse_arr!([u32; 30]);
impl_desse_arr!([u32; 31]);
impl_desse_arr!([u32; 32]);

impl_desse_arr!([u64; 1]);
impl_desse_arr!([u64; 2]);
impl_desse_arr!([u64; 3]);
impl_desse_arr!([u64; 4]);
impl_desse_arr!([u64; 5]);
impl_desse_arr!([u64; 6]);
impl_desse_arr!([u64; 7]);
impl_desse_arr!([u64; 8]);
impl_desse_arr!([u64; 9]);
impl_desse_arr!([u64; 10]);
impl_desse_arr!([u64; 11]);
impl_desse_arr!([u64; 12]);
impl_desse_arr!([u64; 13]);
impl_desse_arr!([u64; 14]);
impl_desse_arr!([u64; 15]);
impl_desse_arr!([u64; 16]);
impl_desse_arr!([u64; 17]);
impl_desse_arr!([u64; 18]);
impl_desse_arr!([u64; 19]);
impl_desse_arr!([u64; 20]);
impl_desse_arr!([u64; 21]);
impl_desse_arr!([u64; 22]);
impl_desse_arr!([u64; 23]);
impl_desse_arr!([u64; 24]);
impl_desse_arr!([u64; 25]);
impl_desse_arr!([u64; 26]);
impl_desse_arr!([u64; 27]);
impl_desse_arr!([u64; 28]);
impl_desse_arr!([u64; 29]);
impl_desse_arr!([u64; 30]);
impl_desse_arr!([u64; 31]);
impl_desse_arr!([u64; 32]);

impl_desse_arr!([u128; 1]);
impl_desse_arr!([u128; 2]);
impl_desse_arr!([u128; 3]);
impl_desse_arr!([u128; 4]);
impl_desse_arr!([u128; 5]);
impl_desse_arr!([u128; 6]);
impl_desse_arr!([u128; 7]);
impl_desse_arr!([u128; 8]);
impl_desse_arr!([u128; 9]);
impl_desse_arr!([u128; 10]);
impl_desse_arr!([u128; 11]);
impl_desse_arr!([u128; 12]);
impl_desse_arr!([u128; 13]);
impl_desse_arr!([u128; 14]);
impl_desse_arr!([u128; 15]);
impl_desse_arr!([u128; 16]);
impl_desse_arr!([u128; 17]);
impl_desse_arr!([u128; 18]);
impl_desse_arr!([u128; 19]);
impl_desse_arr!([u128; 20]);
impl_desse_arr!([u128; 21]);
impl_desse_arr!([u128; 22]);
impl_desse_arr!([u128; 23]);
impl_desse_arr!([u128; 24]);
impl_desse_arr!([u128; 25]);
impl_desse_arr!([u128; 26]);
impl_desse_arr!([u128; 27]);
impl_desse_arr!([u128; 28]);
impl_desse_arr!([u128; 29]);
impl_desse_arr!([u128; 30]);
impl_desse_arr!([u128; 31]);
impl_desse_arr!([u128; 32]);

impl_desse_arr!([i8; 1]);
impl_desse_arr!([i8; 2]);
impl_desse_arr!([i8; 3]);
impl_desse_arr!([i8; 4]);
impl_desse_arr!([i8; 5]);
impl_desse_arr!([i8; 6]);
impl_desse_arr!([i8; 7]);
impl_desse_arr!([i8; 8]);
impl_desse_arr!([i8; 9]);
impl_desse_arr!([i8; 10]);
impl_desse_arr!([i8; 11]);
impl_desse_arr!([i8; 12]);
impl_desse_arr!([i8; 13]);
impl_desse_arr!([i8; 14]);
impl_desse_arr!([i8; 15]);
impl_desse_arr!([i8; 16]);
impl_desse_arr!([i8; 17]);
impl_desse_arr!([i8; 18]);
impl_desse_arr!([i8; 19]);
impl_desse_arr!([i8; 20]);
impl_desse_arr!([i8; 21]);
impl_desse_arr!([i8; 22]);
impl_desse_arr!([i8; 23]);
impl_desse_arr!([i8; 24]);
impl_desse_arr!([i8; 25]);
impl_desse_arr!([i8; 26]);
impl_desse_arr!([i8; 27]);
impl_desse_arr!([i8; 28]);
impl_desse_arr!([i8; 29]);
impl_desse_arr!([i8; 30]);
impl_desse_arr!([i8; 31]);
impl_desse_arr!([i8; 32]);

impl_desse_arr!([i16; 1]);
impl_desse_arr!([i16; 2]);
impl_desse_arr!([i16; 3]);
impl_desse_arr!([i16; 4]);
impl_desse_arr!([i16; 5]);
impl_desse_arr!([i16; 6]);
impl_desse_arr!([i16; 7]);
impl_desse_arr!([i16; 8]);
impl_desse_arr!([i16; 9]);
impl_desse_arr!([i16; 10]);
impl_desse_arr!([i16; 11]);
impl_desse_arr!([i16; 12]);
impl_desse_arr!([i16; 13]);
impl_desse_arr!([i16; 14]);
impl_desse_arr!([i16; 15]);
impl_desse_arr!([i16; 16]);
impl_desse_arr!([i16; 17]);
impl_desse_arr!([i16; 18]);
impl_desse_arr!([i16; 19]);
impl_desse_arr!([i16; 20]);
impl_desse_arr!([i16; 21]);
impl_desse_arr!([i16; 22]);
impl_desse_arr!([i16; 23]);
impl_desse_arr!([i16; 24]);
impl_desse_arr!([i16; 25]);
impl_desse_arr!([i16; 26]);
impl_desse_arr!([i16; 27]);
impl_desse_arr!([i16; 28]);
impl_desse_arr!([i16; 29]);
impl_desse_arr!([i16; 30]);
impl_desse_arr!([i16; 31]);
impl_desse_arr!([i16; 32]);

impl_desse_arr!([i32; 1]);
impl_desse_arr!([i32; 2]);
impl_desse_arr!([i32; 3]);
impl_desse_arr!([i32; 4]);
impl_desse_arr!([i32; 5]);
impl_desse_arr!([i32; 6]);
impl_desse_arr!([i32; 7]);
impl_desse_arr!([i32; 8]);
impl_desse_arr!([i32; 9]);
impl_desse_arr!([i32; 10]);
impl_desse_arr!([i32; 11]);
impl_desse_arr!([i32; 12]);
impl_desse_arr!([i32; 13]);
impl_desse_arr!([i32; 14]);
impl_desse_arr!([i32; 15]);
impl_desse_arr!([i32; 16]);
impl_desse_arr!([i32; 17]);
impl_desse_arr!([i32; 18]);
impl_desse_arr!([i32; 19]);
impl_desse_arr!([i32; 20]);
impl_desse_arr!([i32; 21]);
impl_desse_arr!([i32; 22]);
impl_desse_arr!([i32; 23]);
impl_desse_arr!([i32; 24]);
impl_desse_arr!([i32; 25]);
impl_desse_arr!([i32; 26]);
impl_desse_arr!([i32; 27]);
impl_desse_arr!([i32; 28]);
impl_desse_arr!([i32; 29]);
impl_desse_arr!([i32; 30]);
impl_desse_arr!([i32; 31]);
impl_desse_arr!([i32; 32]);

impl_desse_arr!([i64; 1]);
impl_desse_arr!([i64; 2]);
impl_desse_arr!([i64; 3]);
impl_desse_arr!([i64; 4]);
impl_desse_arr!([i64; 5]);
impl_desse_arr!([i64; 6]);
impl_desse_arr!([i64; 7]);
impl_desse_arr!([i64; 8]);
impl_desse_arr!([i64; 9]);
impl_desse_arr!([i64; 10]);
impl_desse_arr!([i64; 11]);
impl_desse_arr!([i64; 12]);
impl_desse_arr!([i64; 13]);
impl_desse_arr!([i64; 14]);
impl_desse_arr!([i64; 15]);
impl_desse_arr!([i64; 16]);
impl_desse_arr!([i64; 17]);
impl_desse_arr!([i64; 18]);
impl_desse_arr!([i64; 19]);
impl_desse_arr!([i64; 20]);
impl_desse_arr!([i64; 21]);
impl_desse_arr!([i64; 22]);
impl_desse_arr!([i64; 23]);
impl_desse_arr!([i64; 24]);
impl_desse_arr!([i64; 25]);
impl_desse_arr!([i64; 26]);
impl_desse_arr!([i64; 27]);
impl_desse_arr!([i64; 28]);
impl_desse_arr!([i64; 29]);
impl_desse_arr!([i64; 30]);
impl_desse_arr!([i64; 31]);
impl_desse_arr!([i64; 32]);

impl_desse_arr!([i128; 1]);
impl_desse_arr!([i128; 2]);
impl_desse_arr!([i128; 3]);
impl_desse_arr!([i128; 4]);
impl_desse_arr!([i128; 5]);
impl_desse_arr!([i128; 6]);
impl_desse_arr!([i128; 7]);
impl_desse_arr!([i128; 8]);
impl_desse_arr!([i128; 9]);
impl_desse_arr!([i128; 10]);
impl_desse_arr!([i128; 11]);
impl_desse_arr!([i128; 12]);
impl_desse_arr!([i128; 13]);
impl_desse_arr!([i128; 14]);
impl_desse_arr!([i128; 15]);
impl_desse_arr!([i128; 16]);
impl_desse_arr!([i128; 17]);
impl_desse_arr!([i128; 18]);
impl_desse_arr!([i128; 19]);
impl_desse_arr!([i128; 20]);
impl_desse_arr!([i128; 21]);
impl_desse_arr!([i128; 22]);
impl_desse_arr!([i128; 23]);
impl_desse_arr!([i128; 24]);
impl_desse_arr!([i128; 25]);
impl_desse_arr!([i128; 26]);
impl_desse_arr!([i128; 27]);
impl_desse_arr!([i128; 28]);
impl_desse_arr!([i128; 29]);
impl_desse_arr!([i128; 30]);
impl_desse_arr!([i128; 31]);
impl_desse_arr!([i128; 32]);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_desse_test {
        ($type: ty, $name: ident) => {
            #[test]
            fn $name() {
                let num: $type = rand::random::<$type>();
                let new_num = <$type>::deserialize_from(&Desse::serialize(&num)).unwrap();
                assert_eq!(num, new_num, "Invalid serialization / deserialization")
            }
        };
    }

    impl_desse_test!(bool, check_primitive_bool);
    impl_desse_test!(char, check_primitive_char);

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

    impl_desse_test!([bool; 1], check_arr_bool_1);
    impl_desse_test!([bool; 2], check_arr_bool_2);
    impl_desse_test!([bool; 3], check_arr_bool_3);
    impl_desse_test!([bool; 4], check_arr_bool_4);
    impl_desse_test!([bool; 5], check_arr_bool_5);
    impl_desse_test!([bool; 6], check_arr_bool_6);
    impl_desse_test!([bool; 7], check_arr_bool_7);
    impl_desse_test!([bool; 8], check_arr_bool_8);
    impl_desse_test!([bool; 9], check_arr_bool_9);
    impl_desse_test!([bool; 10], check_arr_bool_10);
    impl_desse_test!([bool; 11], check_arr_bool_11);
    impl_desse_test!([bool; 12], check_arr_bool_12);
    impl_desse_test!([bool; 13], check_arr_bool_13);
    impl_desse_test!([bool; 14], check_arr_bool_14);
    impl_desse_test!([bool; 15], check_arr_bool_15);
    impl_desse_test!([bool; 16], check_arr_bool_16);
    impl_desse_test!([bool; 17], check_arr_bool_17);
    impl_desse_test!([bool; 18], check_arr_bool_18);
    impl_desse_test!([bool; 19], check_arr_bool_19);
    impl_desse_test!([bool; 20], check_arr_bool_20);
    impl_desse_test!([bool; 21], check_arr_bool_21);
    impl_desse_test!([bool; 22], check_arr_bool_22);
    impl_desse_test!([bool; 23], check_arr_bool_23);
    impl_desse_test!([bool; 24], check_arr_bool_24);
    impl_desse_test!([bool; 25], check_arr_bool_25);
    impl_desse_test!([bool; 26], check_arr_bool_26);
    impl_desse_test!([bool; 27], check_arr_bool_27);
    impl_desse_test!([bool; 28], check_arr_bool_28);
    impl_desse_test!([bool; 29], check_arr_bool_29);
    impl_desse_test!([bool; 30], check_arr_bool_30);
    impl_desse_test!([bool; 31], check_arr_bool_31);
    impl_desse_test!([bool; 32], check_arr_bool_32);

    impl_desse_test!([char; 1], check_arr_char_1);
    impl_desse_test!([char; 2], check_arr_char_2);
    impl_desse_test!([char; 3], check_arr_char_3);
    impl_desse_test!([char; 4], check_arr_char_4);
    impl_desse_test!([char; 5], check_arr_char_5);
    impl_desse_test!([char; 6], check_arr_char_6);
    impl_desse_test!([char; 7], check_arr_char_7);
    impl_desse_test!([char; 8], check_arr_char_8);
    impl_desse_test!([char; 9], check_arr_char_9);
    impl_desse_test!([char; 10], check_arr_char_10);
    impl_desse_test!([char; 11], check_arr_char_11);
    impl_desse_test!([char; 12], check_arr_char_12);
    impl_desse_test!([char; 13], check_arr_char_13);
    impl_desse_test!([char; 14], check_arr_char_14);
    impl_desse_test!([char; 15], check_arr_char_15);
    impl_desse_test!([char; 16], check_arr_char_16);
    impl_desse_test!([char; 17], check_arr_char_17);
    impl_desse_test!([char; 18], check_arr_char_18);
    impl_desse_test!([char; 19], check_arr_char_19);
    impl_desse_test!([char; 20], check_arr_char_20);
    impl_desse_test!([char; 21], check_arr_char_21);
    impl_desse_test!([char; 22], check_arr_char_22);
    impl_desse_test!([char; 23], check_arr_char_23);
    impl_desse_test!([char; 24], check_arr_char_24);
    impl_desse_test!([char; 25], check_arr_char_25);
    impl_desse_test!([char; 26], check_arr_char_26);
    impl_desse_test!([char; 27], check_arr_char_27);
    impl_desse_test!([char; 28], check_arr_char_28);
    impl_desse_test!([char; 29], check_arr_char_29);
    impl_desse_test!([char; 30], check_arr_char_30);
    impl_desse_test!([char; 31], check_arr_char_31);
    impl_desse_test!([char; 32], check_arr_char_32);

    impl_desse_test!([u8; 1], check_arr_u8_1);
    impl_desse_test!([u8; 2], check_arr_u8_2);
    impl_desse_test!([u8; 3], check_arr_u8_3);
    impl_desse_test!([u8; 4], check_arr_u8_4);
    impl_desse_test!([u8; 5], check_arr_u8_5);
    impl_desse_test!([u8; 6], check_arr_u8_6);
    impl_desse_test!([u8; 7], check_arr_u8_7);
    impl_desse_test!([u8; 8], check_arr_u8_8);
    impl_desse_test!([u8; 9], check_arr_u8_9);
    impl_desse_test!([u8; 10], check_arr_u8_10);
    impl_desse_test!([u8; 11], check_arr_u8_11);
    impl_desse_test!([u8; 12], check_arr_u8_12);
    impl_desse_test!([u8; 13], check_arr_u8_13);
    impl_desse_test!([u8; 14], check_arr_u8_14);
    impl_desse_test!([u8; 15], check_arr_u8_15);
    impl_desse_test!([u8; 16], check_arr_u8_16);
    impl_desse_test!([u8; 17], check_arr_u8_17);
    impl_desse_test!([u8; 18], check_arr_u8_18);
    impl_desse_test!([u8; 19], check_arr_u8_19);
    impl_desse_test!([u8; 20], check_arr_u8_20);
    impl_desse_test!([u8; 21], check_arr_u8_21);
    impl_desse_test!([u8; 22], check_arr_u8_22);
    impl_desse_test!([u8; 23], check_arr_u8_23);
    impl_desse_test!([u8; 24], check_arr_u8_24);
    impl_desse_test!([u8; 25], check_arr_u8_25);
    impl_desse_test!([u8; 26], check_arr_u8_26);
    impl_desse_test!([u8; 27], check_arr_u8_27);
    impl_desse_test!([u8; 28], check_arr_u8_28);
    impl_desse_test!([u8; 29], check_arr_u8_29);
    impl_desse_test!([u8; 30], check_arr_u8_30);
    impl_desse_test!([u8; 31], check_arr_u8_31);
    impl_desse_test!([u8; 32], check_arr_u8_32);

    impl_desse_test!([u16; 1], check_arr_u16_1);
    impl_desse_test!([u16; 2], check_arr_u16_2);
    impl_desse_test!([u16; 3], check_arr_u16_3);
    impl_desse_test!([u16; 4], check_arr_u16_4);
    impl_desse_test!([u16; 5], check_arr_u16_5);
    impl_desse_test!([u16; 6], check_arr_u16_6);
    impl_desse_test!([u16; 7], check_arr_u16_7);
    impl_desse_test!([u16; 8], check_arr_u16_8);
    impl_desse_test!([u16; 9], check_arr_u16_9);
    impl_desse_test!([u16; 10], check_arr_u16_10);
    impl_desse_test!([u16; 11], check_arr_u16_11);
    impl_desse_test!([u16; 12], check_arr_u16_12);
    impl_desse_test!([u16; 13], check_arr_u16_13);
    impl_desse_test!([u16; 14], check_arr_u16_14);
    impl_desse_test!([u16; 15], check_arr_u16_15);
    impl_desse_test!([u16; 16], check_arr_u16_16);
    impl_desse_test!([u16; 17], check_arr_u16_17);
    impl_desse_test!([u16; 18], check_arr_u16_18);
    impl_desse_test!([u16; 19], check_arr_u16_19);
    impl_desse_test!([u16; 20], check_arr_u16_20);
    impl_desse_test!([u16; 21], check_arr_u16_21);
    impl_desse_test!([u16; 22], check_arr_u16_22);
    impl_desse_test!([u16; 23], check_arr_u16_23);
    impl_desse_test!([u16; 24], check_arr_u16_24);
    impl_desse_test!([u16; 25], check_arr_u16_25);
    impl_desse_test!([u16; 26], check_arr_u16_26);
    impl_desse_test!([u16; 27], check_arr_u16_27);
    impl_desse_test!([u16; 28], check_arr_u16_28);
    impl_desse_test!([u16; 29], check_arr_u16_29);
    impl_desse_test!([u16; 30], check_arr_u16_30);
    impl_desse_test!([u16; 31], check_arr_u16_31);
    impl_desse_test!([u16; 32], check_arr_u16_32);

    impl_desse_test!([u32; 1], check_arr_u32_1);
    impl_desse_test!([u32; 2], check_arr_u32_2);
    impl_desse_test!([u32; 3], check_arr_u32_3);
    impl_desse_test!([u32; 4], check_arr_u32_4);
    impl_desse_test!([u32; 5], check_arr_u32_5);
    impl_desse_test!([u32; 6], check_arr_u32_6);
    impl_desse_test!([u32; 7], check_arr_u32_7);
    impl_desse_test!([u32; 8], check_arr_u32_8);
    impl_desse_test!([u32; 9], check_arr_u32_9);
    impl_desse_test!([u32; 10], check_arr_u32_10);
    impl_desse_test!([u32; 11], check_arr_u32_11);
    impl_desse_test!([u32; 12], check_arr_u32_12);
    impl_desse_test!([u32; 13], check_arr_u32_13);
    impl_desse_test!([u32; 14], check_arr_u32_14);
    impl_desse_test!([u32; 15], check_arr_u32_15);
    impl_desse_test!([u32; 16], check_arr_u32_16);
    impl_desse_test!([u32; 17], check_arr_u32_17);
    impl_desse_test!([u32; 18], check_arr_u32_18);
    impl_desse_test!([u32; 19], check_arr_u32_19);
    impl_desse_test!([u32; 20], check_arr_u32_20);
    impl_desse_test!([u32; 21], check_arr_u32_21);
    impl_desse_test!([u32; 22], check_arr_u32_22);
    impl_desse_test!([u32; 23], check_arr_u32_23);
    impl_desse_test!([u32; 24], check_arr_u32_24);
    impl_desse_test!([u32; 25], check_arr_u32_25);
    impl_desse_test!([u32; 26], check_arr_u32_26);
    impl_desse_test!([u32; 27], check_arr_u32_27);
    impl_desse_test!([u32; 28], check_arr_u32_28);
    impl_desse_test!([u32; 29], check_arr_u32_29);
    impl_desse_test!([u32; 30], check_arr_u32_30);
    impl_desse_test!([u32; 31], check_arr_u32_31);
    impl_desse_test!([u32; 32], check_arr_u32_32);

    impl_desse_test!([u64; 1], check_arr_u64_1);
    impl_desse_test!([u64; 2], check_arr_u64_2);
    impl_desse_test!([u64; 3], check_arr_u64_3);
    impl_desse_test!([u64; 4], check_arr_u64_4);
    impl_desse_test!([u64; 5], check_arr_u64_5);
    impl_desse_test!([u64; 6], check_arr_u64_6);
    impl_desse_test!([u64; 7], check_arr_u64_7);
    impl_desse_test!([u64; 8], check_arr_u64_8);
    impl_desse_test!([u64; 9], check_arr_u64_9);
    impl_desse_test!([u64; 10], check_arr_u64_10);
    impl_desse_test!([u64; 11], check_arr_u64_11);
    impl_desse_test!([u64; 12], check_arr_u64_12);
    impl_desse_test!([u64; 13], check_arr_u64_13);
    impl_desse_test!([u64; 14], check_arr_u64_14);
    impl_desse_test!([u64; 15], check_arr_u64_15);
    impl_desse_test!([u64; 16], check_arr_u64_16);
    impl_desse_test!([u64; 17], check_arr_u64_17);
    impl_desse_test!([u64; 18], check_arr_u64_18);
    impl_desse_test!([u64; 19], check_arr_u64_19);
    impl_desse_test!([u64; 20], check_arr_u64_20);
    impl_desse_test!([u64; 21], check_arr_u64_21);
    impl_desse_test!([u64; 22], check_arr_u64_22);
    impl_desse_test!([u64; 23], check_arr_u64_23);
    impl_desse_test!([u64; 24], check_arr_u64_24);
    impl_desse_test!([u64; 25], check_arr_u64_25);
    impl_desse_test!([u64; 26], check_arr_u64_26);
    impl_desse_test!([u64; 27], check_arr_u64_27);
    impl_desse_test!([u64; 28], check_arr_u64_28);
    impl_desse_test!([u64; 29], check_arr_u64_29);
    impl_desse_test!([u64; 30], check_arr_u64_30);
    impl_desse_test!([u64; 31], check_arr_u64_31);
    impl_desse_test!([u64; 32], check_arr_u64_32);

    impl_desse_test!([u128; 1], check_arr_u128_1);
    impl_desse_test!([u128; 2], check_arr_u128_2);
    impl_desse_test!([u128; 3], check_arr_u128_3);
    impl_desse_test!([u128; 4], check_arr_u128_4);
    impl_desse_test!([u128; 5], check_arr_u128_5);
    impl_desse_test!([u128; 6], check_arr_u128_6);
    impl_desse_test!([u128; 7], check_arr_u128_7);
    impl_desse_test!([u128; 8], check_arr_u128_8);
    impl_desse_test!([u128; 9], check_arr_u128_9);
    impl_desse_test!([u128; 10], check_arr_u128_10);
    impl_desse_test!([u128; 11], check_arr_u128_11);
    impl_desse_test!([u128; 12], check_arr_u128_12);
    impl_desse_test!([u128; 13], check_arr_u128_13);
    impl_desse_test!([u128; 14], check_arr_u128_14);
    impl_desse_test!([u128; 15], check_arr_u128_15);
    impl_desse_test!([u128; 16], check_arr_u128_16);
    impl_desse_test!([u128; 17], check_arr_u128_17);
    impl_desse_test!([u128; 18], check_arr_u128_18);
    impl_desse_test!([u128; 19], check_arr_u128_19);
    impl_desse_test!([u128; 20], check_arr_u128_20);
    impl_desse_test!([u128; 21], check_arr_u128_21);
    impl_desse_test!([u128; 22], check_arr_u128_22);
    impl_desse_test!([u128; 23], check_arr_u128_23);
    impl_desse_test!([u128; 24], check_arr_u128_24);
    impl_desse_test!([u128; 25], check_arr_u128_25);
    impl_desse_test!([u128; 26], check_arr_u128_26);
    impl_desse_test!([u128; 27], check_arr_u128_27);
    impl_desse_test!([u128; 28], check_arr_u128_28);
    impl_desse_test!([u128; 29], check_arr_u128_29);
    impl_desse_test!([u128; 30], check_arr_u128_30);
    impl_desse_test!([u128; 31], check_arr_u128_31);
    impl_desse_test!([u128; 32], check_arr_u128_32);

    impl_desse_test!([i8; 1], check_arr_i8_1);
    impl_desse_test!([i8; 2], check_arr_i8_2);
    impl_desse_test!([i8; 3], check_arr_i8_3);
    impl_desse_test!([i8; 4], check_arr_i8_4);
    impl_desse_test!([i8; 5], check_arr_i8_5);
    impl_desse_test!([i8; 6], check_arr_i8_6);
    impl_desse_test!([i8; 7], check_arr_i8_7);
    impl_desse_test!([i8; 8], check_arr_i8_8);
    impl_desse_test!([i8; 9], check_arr_i8_9);
    impl_desse_test!([i8; 10], check_arr_i8_10);
    impl_desse_test!([i8; 11], check_arr_i8_11);
    impl_desse_test!([i8; 12], check_arr_i8_12);
    impl_desse_test!([i8; 13], check_arr_i8_13);
    impl_desse_test!([i8; 14], check_arr_i8_14);
    impl_desse_test!([i8; 15], check_arr_i8_15);
    impl_desse_test!([i8; 16], check_arr_i8_16);
    impl_desse_test!([i8; 17], check_arr_i8_17);
    impl_desse_test!([i8; 18], check_arr_i8_18);
    impl_desse_test!([i8; 19], check_arr_i8_19);
    impl_desse_test!([i8; 20], check_arr_i8_20);
    impl_desse_test!([i8; 21], check_arr_i8_21);
    impl_desse_test!([i8; 22], check_arr_i8_22);
    impl_desse_test!([i8; 23], check_arr_i8_23);
    impl_desse_test!([i8; 24], check_arr_i8_24);
    impl_desse_test!([i8; 25], check_arr_i8_25);
    impl_desse_test!([i8; 26], check_arr_i8_26);
    impl_desse_test!([i8; 27], check_arr_i8_27);
    impl_desse_test!([i8; 28], check_arr_i8_28);
    impl_desse_test!([i8; 29], check_arr_i8_29);
    impl_desse_test!([i8; 30], check_arr_i8_30);
    impl_desse_test!([i8; 31], check_arr_i8_31);
    impl_desse_test!([i8; 32], check_arr_i8_32);

    impl_desse_test!([i16; 1], check_arr_i16_1);
    impl_desse_test!([i16; 2], check_arr_i16_2);
    impl_desse_test!([i16; 3], check_arr_i16_3);
    impl_desse_test!([i16; 4], check_arr_i16_4);
    impl_desse_test!([i16; 5], check_arr_i16_5);
    impl_desse_test!([i16; 6], check_arr_i16_6);
    impl_desse_test!([i16; 7], check_arr_i16_7);
    impl_desse_test!([i16; 8], check_arr_i16_8);
    impl_desse_test!([i16; 9], check_arr_i16_9);
    impl_desse_test!([i16; 10], check_arr_i16_10);
    impl_desse_test!([i16; 11], check_arr_i16_11);
    impl_desse_test!([i16; 12], check_arr_i16_12);
    impl_desse_test!([i16; 13], check_arr_i16_13);
    impl_desse_test!([i16; 14], check_arr_i16_14);
    impl_desse_test!([i16; 15], check_arr_i16_15);
    impl_desse_test!([i16; 16], check_arr_i16_16);
    impl_desse_test!([i16; 17], check_arr_i16_17);
    impl_desse_test!([i16; 18], check_arr_i16_18);
    impl_desse_test!([i16; 19], check_arr_i16_19);
    impl_desse_test!([i16; 20], check_arr_i16_20);
    impl_desse_test!([i16; 21], check_arr_i16_21);
    impl_desse_test!([i16; 22], check_arr_i16_22);
    impl_desse_test!([i16; 23], check_arr_i16_23);
    impl_desse_test!([i16; 24], check_arr_i16_24);
    impl_desse_test!([i16; 25], check_arr_i16_25);
    impl_desse_test!([i16; 26], check_arr_i16_26);
    impl_desse_test!([i16; 27], check_arr_i16_27);
    impl_desse_test!([i16; 28], check_arr_i16_28);
    impl_desse_test!([i16; 29], check_arr_i16_29);
    impl_desse_test!([i16; 30], check_arr_i16_30);
    impl_desse_test!([i16; 31], check_arr_i16_31);
    impl_desse_test!([i16; 32], check_arr_i16_32);

    impl_desse_test!([i32; 1], check_arr_i32_1);
    impl_desse_test!([i32; 2], check_arr_i32_2);
    impl_desse_test!([i32; 3], check_arr_i32_3);
    impl_desse_test!([i32; 4], check_arr_i32_4);
    impl_desse_test!([i32; 5], check_arr_i32_5);
    impl_desse_test!([i32; 6], check_arr_i32_6);
    impl_desse_test!([i32; 7], check_arr_i32_7);
    impl_desse_test!([i32; 8], check_arr_i32_8);
    impl_desse_test!([i32; 9], check_arr_i32_9);
    impl_desse_test!([i32; 10], check_arr_i32_10);
    impl_desse_test!([i32; 11], check_arr_i32_11);
    impl_desse_test!([i32; 12], check_arr_i32_12);
    impl_desse_test!([i32; 13], check_arr_i32_13);
    impl_desse_test!([i32; 14], check_arr_i32_14);
    impl_desse_test!([i32; 15], check_arr_i32_15);
    impl_desse_test!([i32; 16], check_arr_i32_16);
    impl_desse_test!([i32; 17], check_arr_i32_17);
    impl_desse_test!([i32; 18], check_arr_i32_18);
    impl_desse_test!([i32; 19], check_arr_i32_19);
    impl_desse_test!([i32; 20], check_arr_i32_20);
    impl_desse_test!([i32; 21], check_arr_i32_21);
    impl_desse_test!([i32; 22], check_arr_i32_22);
    impl_desse_test!([i32; 23], check_arr_i32_23);
    impl_desse_test!([i32; 24], check_arr_i32_24);
    impl_desse_test!([i32; 25], check_arr_i32_25);
    impl_desse_test!([i32; 26], check_arr_i32_26);
    impl_desse_test!([i32; 27], check_arr_i32_27);
    impl_desse_test!([i32; 28], check_arr_i32_28);
    impl_desse_test!([i32; 29], check_arr_i32_29);
    impl_desse_test!([i32; 30], check_arr_i32_30);
    impl_desse_test!([i32; 31], check_arr_i32_31);
    impl_desse_test!([i32; 32], check_arr_i32_32);

    impl_desse_test!([i64; 1], check_arr_i64_1);
    impl_desse_test!([i64; 2], check_arr_i64_2);
    impl_desse_test!([i64; 3], check_arr_i64_3);
    impl_desse_test!([i64; 4], check_arr_i64_4);
    impl_desse_test!([i64; 5], check_arr_i64_5);
    impl_desse_test!([i64; 6], check_arr_i64_6);
    impl_desse_test!([i64; 7], check_arr_i64_7);
    impl_desse_test!([i64; 8], check_arr_i64_8);
    impl_desse_test!([i64; 9], check_arr_i64_9);
    impl_desse_test!([i64; 10], check_arr_i64_10);
    impl_desse_test!([i64; 11], check_arr_i64_11);
    impl_desse_test!([i64; 12], check_arr_i64_12);
    impl_desse_test!([i64; 13], check_arr_i64_13);
    impl_desse_test!([i64; 14], check_arr_i64_14);
    impl_desse_test!([i64; 15], check_arr_i64_15);
    impl_desse_test!([i64; 16], check_arr_i64_16);
    impl_desse_test!([i64; 17], check_arr_i64_17);
    impl_desse_test!([i64; 18], check_arr_i64_18);
    impl_desse_test!([i64; 19], check_arr_i64_19);
    impl_desse_test!([i64; 20], check_arr_i64_20);
    impl_desse_test!([i64; 21], check_arr_i64_21);
    impl_desse_test!([i64; 22], check_arr_i64_22);
    impl_desse_test!([i64; 23], check_arr_i64_23);
    impl_desse_test!([i64; 24], check_arr_i64_24);
    impl_desse_test!([i64; 25], check_arr_i64_25);
    impl_desse_test!([i64; 26], check_arr_i64_26);
    impl_desse_test!([i64; 27], check_arr_i64_27);
    impl_desse_test!([i64; 28], check_arr_i64_28);
    impl_desse_test!([i64; 29], check_arr_i64_29);
    impl_desse_test!([i64; 30], check_arr_i64_30);
    impl_desse_test!([i64; 31], check_arr_i64_31);
    impl_desse_test!([i64; 32], check_arr_i64_32);

    impl_desse_test!([i128; 1], check_arr_i128_1);
    impl_desse_test!([i128; 2], check_arr_i128_2);
    impl_desse_test!([i128; 3], check_arr_i128_3);
    impl_desse_test!([i128; 4], check_arr_i128_4);
    impl_desse_test!([i128; 5], check_arr_i128_5);
    impl_desse_test!([i128; 6], check_arr_i128_6);
    impl_desse_test!([i128; 7], check_arr_i128_7);
    impl_desse_test!([i128; 8], check_arr_i128_8);
    impl_desse_test!([i128; 9], check_arr_i128_9);
    impl_desse_test!([i128; 10], check_arr_i128_10);
    impl_desse_test!([i128; 11], check_arr_i128_11);
    impl_desse_test!([i128; 12], check_arr_i128_12);
    impl_desse_test!([i128; 13], check_arr_i128_13);
    impl_desse_test!([i128; 14], check_arr_i128_14);
    impl_desse_test!([i128; 15], check_arr_i128_15);
    impl_desse_test!([i128; 16], check_arr_i128_16);
    impl_desse_test!([i128; 17], check_arr_i128_17);
    impl_desse_test!([i128; 18], check_arr_i128_18);
    impl_desse_test!([i128; 19], check_arr_i128_19);
    impl_desse_test!([i128; 20], check_arr_i128_20);
    impl_desse_test!([i128; 21], check_arr_i128_21);
    impl_desse_test!([i128; 22], check_arr_i128_22);
    impl_desse_test!([i128; 23], check_arr_i128_23);
    impl_desse_test!([i128; 24], check_arr_i128_24);
    impl_desse_test!([i128; 25], check_arr_i128_25);
    impl_desse_test!([i128; 26], check_arr_i128_26);
    impl_desse_test!([i128; 27], check_arr_i128_27);
    impl_desse_test!([i128; 28], check_arr_i128_28);
    impl_desse_test!([i128; 29], check_arr_i128_29);
    impl_desse_test!([i128; 30], check_arr_i128_30);
    impl_desse_test!([i128; 31], check_arr_i128_31);
    impl_desse_test!([i128; 32], check_arr_i128_32);
}
