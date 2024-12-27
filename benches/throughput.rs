use std::iter;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_core::{RngCore, SeedableRng};
use rand_murmur3::{Mixer32, Mixer64, Seed32, Seed64};

#[derive(Clone, Copy, Debug)]
struct NoopRng;

#[derive(Clone, Copy, Debug)]
struct CountingRng {
    state: usize,
}

impl RngCore for NoopRng {
    fn next_u32(&mut self) -> u32 {
        0
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl RngCore for CountingRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let n = self.state;

        self.state += 1;

        n as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn throughput<R: RngCore>(mut rng: R, size: usize) {
    let mut buff = iter::repeat(0).take(size).collect::<Vec<_>>();

    rng.fill_bytes(&mut buff);
}

fn noop(c: &mut Criterion) {
    let rng = NoopRng;

    c.bench_function("noop rng", |bench| {
        bench.iter(|| throughput::<NoopRng>(black_box(rng), black_box(4000000)))
    });
}

fn counter(c: &mut Criterion) {
    let rng = CountingRng { state: 0 };

    c.bench_function("counting rng", |bench| {
        bench.iter(|| throughput::<CountingRng>(black_box(rng), black_box(4000000)))
    });
}

fn mixer32(c: &mut Criterion) {
    let seed = Seed32(10u32.to_ne_bytes());
    let rng = Mixer32::from_seed(seed);

    c.bench_function("mixer32", |bench| {
        bench.iter(|| throughput::<Mixer32>(black_box(rng), black_box(4000000)))
    });
}

fn mixer64(c: &mut Criterion) {
    let seed = Seed64(10u64.to_ne_bytes());
    let rng = Mixer64::from_seed(seed);

    c.bench_function("mixer32", |bench| {
        bench.iter(|| throughput::<Mixer64>(black_box(rng), black_box(4000000)))
    });
}

criterion_group!(benches, noop, counter, mixer32, mixer64);
criterion_main!(benches);
