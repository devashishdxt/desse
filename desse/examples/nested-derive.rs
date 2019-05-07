use desse::{DesseSized, DesseStatic};

#[derive(Debug, PartialEq, DesseStatic, DesseSized)]
struct Inner {
    a: u8,
    b: u16,
    c: MyEnum,
}

#[derive(Debug, PartialEq, DesseStatic, DesseSized)]
struct MyStruct {
    inner: Inner,
}

#[derive(Debug, PartialEq, DesseStatic, DesseSized)]
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

    let new_struct = MyStruct::deserialize_from(&serialized).unwrap();

    println!("De-serialized: {:?}", new_struct);

    assert_eq!(my_struct, new_struct, "Wrong implementation");
    println!("Done!");
}
