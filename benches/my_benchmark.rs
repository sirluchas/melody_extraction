use criterion::{black_box, criterion_group, criterion_main, Criterion};
use melody_extraction as me;
use std::f32::consts::PI;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Salamon");
    let a_wave: Vec<f32> = (0..22050)
        .map(|x| (x as f32 * 440. * 2. * PI / 44100.).sin())
        .collect();

    group.bench_function("stft peak finding", |b| {
        b.iter(|| me::stft_peaks(black_box(&a_wave)))
    });

    let stft_frame_peaks = me::stft_peaks(&a_wave);
    group.bench_function("salience peak finding", |b| {
        b.iter(|| me::salience_peaks(black_box(&stft_frame_peaks)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
