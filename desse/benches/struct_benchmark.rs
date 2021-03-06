#[macro_use]
extern crate serde_derive;

use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};

use bincode::{deserialize, serialize_into};
use desse::{DesseDynamic, DesseSized, DesseStatic};

#[derive(DesseStatic, DesseSized)]
struct MyDesseStruct {
    a: u8,
    b: u16,
    c: MyDesseEnum,
}

#[derive(DesseStatic, DesseSized)]
enum MyDesseEnum {
    Variant1,
    Variant2(u16),
}

#[derive(Serialize, Deserialize)]
struct MySerdeStruct {
    a: u8,
    b: u16,
    c: MySerdeEnum,
}

#[derive(Serialize, Deserialize)]
enum MySerdeEnum {
    Variant1,
    Variant2(u16),
}

#[allow(unused_must_use)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "struct::serialize",
        Benchmark::new("desse::serialize", |b| {
            b.iter(|| {
                let my_struct: MyDesseStruct = MyDesseStruct {
                    a: 253,
                    b: 64016,
                    c: MyDesseEnum::Variant2(64016),
                };
                black_box(DesseStatic::serialize(black_box(&my_struct)));
            })
        })
        .with_function("bincode::serialize", |b| {
            let mut buffer = Vec::new();
            b.iter(|| {
                buffer.clear();
                let my_struct: MySerdeStruct = MySerdeStruct {
                    a: 253,
                    b: 64016,
                    c: MySerdeEnum::Variant2(64016),
                };
                black_box(serialize_into(&mut buffer, black_box(&my_struct)));
            })
        }),
    );

    c.bench(
        "struct::deserialize",
        Benchmark::new("desse::deserialize", |b| {
            b.iter(|| {
                let bytes: [u8; 6] = [253, 16, 250, 1, 16, 250];
                black_box(MyDesseStruct::deserialize_from(black_box(&bytes)));
            })
        })
        .with_function("bincode::deserialize", |b| {
            b.iter(|| {
                let bytes: [u8; 9] = [253, 16, 250, 1, 0, 0, 0, 16, 250];
                black_box(deserialize::<MySerdeStruct>(black_box(&bytes)));
            })
        }),
    );

    c.bench(
        "dynamic::serde",
        Benchmark::new("desse:serde", |b| {
            let v = vec!["hello".to_string(), "world".to_string()];
            b.iter(|| {
                let serialized = DesseDynamic::serialize(&v).unwrap();
                <Vec<String>>::deserialize_from(&*serialized).unwrap();
            })
        })
        .with_function("bincode::serde", |b| {
            let v = vec!["hello".to_string(), "world".to_string()];
            b.iter(|| {
                let serialized = bincode::serialize(&v).unwrap();
                bincode::deserialize::<Vec<String>>(&serialized).unwrap();
            })
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
