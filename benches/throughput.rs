use std::iter;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_core::{RngCore, SeedableRng};
use rand_murmur3::{Mixer32, Mixer64};

const SEED: u64 = 10;
const BUFFER_SIZE: usize = 4000000;

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
        dest.fill(0);
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
    c.bench_function("noop rng", |bench| {
        bench.iter(|| throughput::<NoopRng>(black_box(NoopRng), black_box(BUFFER_SIZE)))
    });
}

fn counter(c: &mut Criterion) {
    c.bench_function("counting rng", |bench| {
        bench.iter(|| {
            throughput::<CountingRng>(black_box(CountingRng { state: 0 }), black_box(BUFFER_SIZE))
        })
    });
}

fn mixer32(c: &mut Criterion) {
    c.bench_function("mixer32", |bench| {
        bench.iter(|| {
            throughput::<Mixer32>(
                black_box(Mixer32::from_seed((SEED as u32).to_ne_bytes())),
                black_box(BUFFER_SIZE),
            )
        })
    });
}

fn mixer64(c: &mut Criterion) {
    c.bench_function("mixer64", |bench| {
        bench.iter(|| {
            throughput::<Mixer64>(
                black_box(Mixer64::from_seed(10u64.to_ne_bytes())),
                black_box(BUFFER_SIZE),
            )
        })
    });
}

fn xorshift(c: &mut Criterion) {
    c.bench_function("xorshift64", |bench| {
        bench.iter(|| {
            throughput(
                black_box(rand_xorshift::XorShiftRng::seed_from_u64(SEED)),
                black_box(BUFFER_SIZE),
            )
        })
    });
}

fn pcg(c: &mut Criterion) {
    c.bench_function("pcg", |bench| {
        bench.iter(|| {
            throughput(
                black_box(rand_pcg::Pcg64::seed_from_u64(SEED)),
                black_box(BUFFER_SIZE),
            )
        })
    });
}

criterion_group!(benches, noop, counter, mixer32, mixer64, xorshift, pcg);
criterion_main!(benches);
