use rand::random;

use desse::{Desse, DesseSized, Result};

#[allow(unused)]
#[derive(Debug, PartialEq, DesseSized)]
enum MyEnum {
    Variant1,
    Variant2(u8, u16),
    Variant3 { a: u32, b: u32 },
}

impl Desse for MyEnum {
    type Output = [u8; Self::SIZE];

    #[inline]
    fn serialize(&self) -> Self::Output {
        let mut bytes: Self::Output = [0; Self::SIZE];
        self.serialize_into(&mut bytes);
        bytes
    }

    #[inline]
    fn serialize_into(&self, bytes: &mut Self::Output) {
        match self {
            MyEnum::Variant1 => (&mut bytes[0..1]).copy_from_slice(&Desse::serialize(&0u8)),
            MyEnum::Variant2(ref a, ref b) => {
                (&mut bytes[0..1]).copy_from_slice(&Desse::serialize(&1u8));
                (&mut bytes[1..2]).copy_from_slice(&Desse::serialize(a));
                (&mut bytes[2..4]).copy_from_slice(&Desse::serialize(b));
            }
            MyEnum::Variant3 { ref a, ref b } => {
                (&mut bytes[0..1]).copy_from_slice(&Desse::serialize(&2u8));
                (&mut bytes[1..5]).copy_from_slice(&Desse::serialize(a));
                (&mut bytes[5..9]).copy_from_slice(&Desse::serialize(b));
            }
        }
    }

    #[inline]
    fn deserialize_from(bytes: &Self::Output) -> Result<Self> {
        let variant =
            unsafe { <u8>::deserialize_from(&*(bytes[0..1].as_ptr() as *const [u8; 1]))? };

        match variant {
            0 => Ok(MyEnum::Variant1),
            1 => unsafe {
                Ok(MyEnum::Variant2(
                    <u8>::deserialize_from(&*(bytes[1..2].as_ptr() as *const [u8; 1]))?,
                    <u16>::deserialize_from(&*(bytes[2..4].as_ptr() as *const [u8; 2]))?,
                ))
            },
            2 => unsafe {
                Ok(MyEnum::Variant3 {
                    a: <u32>::deserialize_from(&*(bytes[1..5].as_ptr() as *const [u8; 4]))?,
                    b: <u32>::deserialize_from(&*(bytes[5..9].as_ptr() as *const [u8; 4]))?,
                })
            },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let my_enum = MyEnum::Variant3 {
        a: random(),
        b: random(),
    };

    println! {"Serialized: {:?}", my_enum.serialize()};

    assert_eq!(
        my_enum,
        MyEnum::deserialize_from(&my_enum.serialize()).unwrap()
    );
}
