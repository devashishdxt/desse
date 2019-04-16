#[macro_use]
extern crate serde_derive;

use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};

use bincode::{deserialize, serialize_into};
use desse::{Desse, DesseSized};

#[derive(Desse, DesseSized)]
struct MyDesseStruct {
    a: u8,
    b: u16,
    c: MyDesseEnum,
}

#[derive(Desse, DesseSized)]
enum MyDesseEnum {
    Variant1,
    Variant2,
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
    Variant2,
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
                    c: MyDesseEnum::Variant1,
                };
                black_box(Desse::serialize(black_box(&my_struct)));
            })
        })
        .with_function("bincode::serialize", |b| {
            let mut buffer = Vec::new();
            b.iter(|| {
                buffer.clear();
                let my_struct: MySerdeStruct = MySerdeStruct {
                    a: 253,
                    b: 64016,
                    c: MySerdeEnum::Variant1,
                };
                black_box(serialize_into(&mut buffer, black_box(&my_struct)));
            })
        }),
    );

    c.bench(
        "struct::deserialize",
        Benchmark::new("desse::deserialize", |b| {
            b.iter(|| {
                let bytes: [u8; 4] = [253, 16, 250, 0];
                black_box(MyDesseStruct::deserialize_from(black_box(&bytes)));
            })
        })
        .with_function("bincode::deserialize", |b| {
            b.iter(|| {
                let bytes: [u8; 7] = [253, 16, 250, 0, 0, 0, 0];
                black_box(deserialize::<MySerdeStruct>(black_box(&bytes)));
            })
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
