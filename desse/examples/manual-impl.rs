use desse::{Desse, DesseSized};

#[derive(Debug, Default, PartialEq)]
struct MyStruct {
    a: u8,
    b: u16,
}

impl DesseSized for MyStruct {
    const SIZE: usize = <u8>::SIZE + <u16>::SIZE;
}

impl Desse for MyStruct {
    type Output = [u8; <MyStruct>::SIZE];

    fn serialize(&self) -> Self::Output {
        let mut bytes: Self::Output = Default::default();
        self.serialize_into(&mut bytes);
        bytes
    }

    fn serialize_into(&self, bytes: &mut Self::Output) {
        unsafe {
            Desse::serialize_into(&self.a, &mut *(bytes[0..1].as_mut_ptr() as *mut [u8; 1]));
            Desse::serialize_into(&self.b, &mut *(bytes[1..3].as_mut_ptr() as *mut [u8; 2]));
        }
    }

    fn deserialize_from(bytes: &Self::Output) -> Self {
        unsafe {
            MyStruct {
                a: <u8>::deserialize_from(&*(bytes[0..1].as_ptr() as *const [u8; 1])),
                b: <u16>::deserialize_from(&*(bytes[1..3].as_ptr() as *const [u8; 2])),
            }
        }
    }
}

fn main() {
    let my_struct = MyStruct {
        a: rand::random(),
        b: rand::random(),
    };

    let serialized = my_struct.serialize();

    println!("Object       : {:?}", my_struct);
    println!("Serialized   : {:?}", serialized);

    let new_struct = MyStruct::deserialize_from(&serialized);

    println!("De-serialized: {:?}", new_struct);

    assert_eq!(my_struct, new_struct, "Wrong implementation");
    println!("Done!");
}
