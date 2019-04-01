use desse::{Desse, DesseSized};

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct Inner {
    a: u8,
    b: u16,
}

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct MyStruct {
    inner: Inner,
}

fn main() {
    let inner = Inner {
        a: rand::random(),
        b: rand::random(),
    };

    let my_struct = MyStruct { inner };

    let serialized = my_struct.serialize();

    println!("Object       : {:?}", my_struct);
    println!("Serialized   : {:?}", serialized);

    let new_struct = MyStruct::deserialize_from(&serialized);

    println!("De-serialized: {:?}", new_struct);

    assert_eq!(my_struct, new_struct, "Wrong implementation");
    println!("Done!");
}
