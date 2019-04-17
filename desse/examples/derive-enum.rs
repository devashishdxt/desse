use desse::{Desse, DesseSized};

#[derive(Debug, PartialEq, DesseSized, Desse)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

#[allow(unused)]
#[derive(Debug, PartialEq, DesseSized)]
enum NonUnitEnum {
    Variant1,
    Variant2(u8, u16),
    Variant3 { a: u64 },
}

fn main() {
    let my_enum = MyEnum::Variant1;

    let serialized = my_enum.serialize();

    println!("Object       : {:?}", my_enum);
    println!("Serialized   : {:?}", serialized);

    let new_enum = MyEnum::deserialize_from(&serialized);

    println!("De-serialized: {:?}", new_enum);

    assert_eq!(my_enum, new_enum, "Wrong implementation");
    assert_eq!(9, NonUnitEnum::SIZE);
    println!("Done!");
}
