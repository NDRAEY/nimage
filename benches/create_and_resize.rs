use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use nimage::{Image, PixelFormat};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_and_scale", |b| b.iter(|| {
        let mut image = Image::new(600, 600, PixelFormat::ARGB);
        image.scale(1920, 1280);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);