use byteorder::{ByteOrder, LittleEndian};
use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use desse::Desse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "primitive::serialize",
        Benchmark::new("desse::serialize", |b| {
            b.iter(|| {
                let num: u128 = 3286752873;
                num.serialize();
            })
        })
        .with_function("byteorder::serialize", |b| {
            b.iter(|| {
                let num: u128 = 3286752873;
                LittleEndian::write_u128(&mut [0; 16], num)
            })
        }),
    );

    c.bench(
        "primitive::deserialize",
        Benchmark::new("desse::deserialize", |b| {
            b.iter(|| {
                let num: u128 = 3286752873;
                let bytes = num.serialize();

                u128::deserialize_from(&bytes);
            })
        })
        .with_function("byteorder::deserialize", |b| {
            b.iter(|| {
                let num: u128 = 3286752873;
                let mut bytes: [u8; 16] = [0; 16];
                LittleEndian::write_u128(&mut bytes, num);

                LittleEndian::read_u128(&bytes);
            })
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
