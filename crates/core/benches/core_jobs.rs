//! Representative core job benchmarks (`criterion`, P1-01).
//!
//! Parallel helpers on realistically sized inputs: finite-sum over a
//! 100k-vertex-scale buffer and channel drain throughput.

use criterion::{Criterion, criterion_group, criterion_main};
use petunia_core::{JobChannel, par_finite_sum};

fn bench_par_finite_sum(c: &mut Criterion) {
    let values: Vec<f32> = (0..100_000).map(|i| (i as f32) * 0.25).collect();
    c.bench_function("core/par_finite_sum_100k", |b| {
        b.iter(|| std::hint::black_box(par_finite_sum(&values)))
    });
}

fn bench_channel_drain(c: &mut Criterion) {
    c.bench_function("core/channel_drain_1k", |b| {
        b.iter(|| {
            let channel: JobChannel<u32> = JobChannel::bounded(1024);
            for i in 0..1024 {
                channel.try_send(i).unwrap();
            }
            std::hint::black_box(channel.drain().len())
        })
    });
}

criterion_group!(benches, bench_par_finite_sum, bench_channel_drain);
criterion_main!(benches);
