use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use desse::{DesseDynamic, DesseSized, DesseStatic, ErrorKind, Result};

#[derive(DesseStatic, DesseSized)]
struct MyStaticStruct {
    a: u8,
    b: u16,
}

struct MyDynamicStruct {
    a: u8,
    b: u16,
}

impl DesseDynamic for MyDynamicStruct {
    type Output = Self;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.a.serialized_size() + self.b.serialized_size()
    }

    #[inline]
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![0; DesseDynamic::serialized_size(self)];

        unsafe {
            DesseDynamic::serialize_into_unchecked(self, &mut bytes)?;
        }

        Ok(bytes)
    }

    #[inline]
    fn serialize_into(&self, bytes: &mut [u8]) -> Result<()> {
        let size = DesseDynamic::serialized_size(self);

        if bytes.len() < size {
            Err(ErrorKind::InvalidSliceLength.into())
        } else {
            unsafe { DesseDynamic::serialize_into_unchecked(self, &mut bytes[0..size]) }
        }
    }

    #[inline]
    unsafe fn serialize_into_unchecked(&self, bytes: &mut [u8]) -> Result<()> {
        let mut counter = 0;

        DesseDynamic::serialize_into_unchecked(
            &self.a,
            &mut bytes[counter..(counter + DesseDynamic::serialized_size(&self.a))],
        )?;
        counter += DesseDynamic::serialized_size(&self.a);

        DesseDynamic::serialize_into_unchecked(
            &self.b,
            &mut bytes[counter..(counter + DesseDynamic::serialized_size(&self.b))],
        )
    }

    #[inline]
    fn deserialize_from(bytes: &[u8]) -> Result<Self::Output> {
        let mut counter = 0;

        let a = <u8 as DesseDynamic>::deserialize_from(&bytes[counter..])?;
        counter += DesseDynamic::serialized_size(&a);

        let b = <u16 as DesseDynamic>::deserialize_from(&bytes[counter..])?;

        Ok(MyDynamicStruct { a, b })
    }

    #[inline]
    unsafe fn deserialize_from_unchecked(bytes: &[u8]) -> Result<Self::Output> {
        let mut counter = 0;

        let a = <u8 as DesseDynamic>::deserialize_from_unchecked(&bytes[counter..])?;
        counter += DesseDynamic::serialized_size(&a);

        let b = <u16 as DesseDynamic>::deserialize_from_unchecked(&bytes[counter..])?;

        Ok(MyDynamicStruct { a, b })
    }
}

#[allow(unused_must_use)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "desse::serialize",
        Benchmark::new("desse::static", |b| {
            let my_struct: MyStaticStruct = MyStaticStruct { a: 253, b: 64016 };

            b.iter(|| {
                black_box(DesseStatic::serialize(black_box(&my_struct)));
            })
        })
        .with_function("desse::dynamic", |b| {
            let my_struct: MyDynamicStruct = MyDynamicStruct { a: 253, b: 64016 };
            let mut bytes = vec![0; DesseDynamic::serialized_size(&my_struct)];

            b.iter(|| {
                black_box(DesseDynamic::serialize_into(
                    black_box(&my_struct),
                    black_box(&mut bytes),
                ));
            })
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
