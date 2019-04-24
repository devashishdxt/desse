use rand::random;

use desse::{Desse, DesseSized};

#[derive(Debug, PartialEq, DesseSized, Desse)]
enum MyEnum {
    Variant1,
    Variant2(u16),
    Variant3(u8),
}

#[allow(unused)]
#[derive(Debug, PartialEq, DesseSized, Desse)]
enum NonUnitEnum {
    Variant1(u128),
    Variant2(u8, u16),
    Variant3 { a: u32, b: MyEnum },
}

fn main() {
    let my_enum = NonUnitEnum::Variant3 {
        a: random(),
        b: MyEnum::Variant3(random()),
    };

    let serialized = my_enum.serialize();

    println!("Size         : {}", NonUnitEnum::SIZE);
    println!("Object       : {:?}", my_enum);
    println!("Serialized   : {:?}", serialized);

    let new_enum = NonUnitEnum::deserialize_from(&serialized).unwrap();

    println!("De-serialized: {:?}", new_enum);

    assert_eq!(my_enum, new_enum, "Wrong implementation");
    println!("Done!");
}
