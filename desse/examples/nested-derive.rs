use desse::{Desse, DesseSized};

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct Inner {
    a: u8,
    b: u16,
    c: MyEnum,
}

#[derive(Debug, PartialEq, Desse, DesseSized)]
struct MyStruct {
    inner: Inner,
}

#[derive(Debug, PartialEq, Desse, DesseSized)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
    let inner = Inner {
        a: rand::random(),
        b: rand::random(),
        c: MyEnum::Variant1,
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
