use desse::{Desse, DesseSized};

#[derive(Debug, Default, PartialEq, Desse, DesseSized)]
struct MyStruct {
    a: u8,
    b: u16,
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

    assert_eq!(
        my_struct,
        new_struct,
        "Wrong implementation"
    );
    println!("Done!");
}
