use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const CRC24_POLY: i32 = 0x1974f0b;
const CRC24_INIT: i32 = 0x875060;

/// Computes crc24 value of `bytes`.
pub fn basic_crc24(bytes: &[u8]) -> i32 {
    bytes.iter().fold(CRC24_INIT, |mut crc, byte| {
        crc ^= (*byte as i32) << 16;

        for _ in 0..8 {
            crc <<= 1;
            if (crc & 0x1000000) != 0 {
                crc ^= CRC24_POLY;
            }
        }

        crc
    })
}

pub fn crc24_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC24");
    for nbytes in [128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536].iter() {
        group.bench_with_input(
            BenchmarkId::new("crc24-openpgp-fast", nbytes),
            nbytes,
            |b, nbytes| b.iter(|| crc24_openpgp_fast::hash_raw(&(b"F".repeat(*nbytes)))),
        );
        group.bench_with_input(BenchmarkId::new("crc24", nbytes), nbytes, |b, nbytes| {
            b.iter(|| crc24::hash_raw(&(b"F".repeat(*nbytes))))
        });
        group.bench_with_input(
            BenchmarkId::new("basic_crc24", nbytes),
            nbytes,
            |b, nbytes| b.iter(|| basic_crc24(&(b"F".repeat(*nbytes)))),
        );
    }
}

pub fn crc32_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("CRC32");
    for nbytes in [128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536].iter() {
        group.bench_with_input(BenchmarkId::new("crc32-v2", nbytes), nbytes, |b, nbytes| {
            b.iter(|| crc32_v2::crc32(0, &(b"F".repeat(*nbytes))))
        });

        group.bench_with_input(BenchmarkId::new("crc32c", nbytes), nbytes, |b, nbytes| {
            b.iter(|| crc32c::crc32c(&(b"F".repeat(*nbytes))))
        });

        group.bench_with_input(
            BenchmarkId::new("crc32fast", nbytes),
            nbytes,
            |b, nbytes| b.iter(|| crc32fast::Hasher::new().update(&(b"F".repeat(*nbytes)))),
        );

        group.bench_with_input(
            BenchmarkId::new("crc32_light", nbytes),
            nbytes,
            |b, nbytes| b.iter(|| crc32_light::crc32(&(b"F".repeat(*nbytes)))),
        );
    }
}

criterion_group!(benches, crc24_benchmarks, crc32_benchmarks);
criterion_main!(benches);
