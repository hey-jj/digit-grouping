//! Gate 6: Buffer formatting versus a naive string-splice baseline.
//!
//! Criterion benches record per-locale timings. After the benches, a
//! median-of-5 measurement computes the u64-under-en ratio. The measured speedup
//! is roughly 5x. The 4x gate catches genuine regressions without flapping on
//! loaded or shared CI runners.

use std::time::Instant;

use criterion::{criterion_group, Criterion};
use digit_grouping::{Buffer, Locale};
use std::hint::black_box;

/// The baseline: format with `to_string`, then splice separators in with
/// `String::insert_str`, walking from the right.
fn naive(value: u64, separator: &str) -> String {
    let mut out = value.to_string();
    let mut index = out.len();
    while index > 3 {
        index -= 3;
        out.insert_str(index, separator);
    }
    out
}

fn values() -> Vec<u64> {
    (0..1000u64)
        .map(|i| i.wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .collect()
}

fn bench_buffer(c: &mut Criterion) {
    let values = values();
    for locale in [Locale::en, Locale::hi, Locale::ar_EG] {
        c.bench_function(&format!("buffer/u64/{locale}"), |b| {
            let mut buffer = Buffer::new();
            b.iter(|| {
                for &v in &values {
                    black_box(buffer.write_formatted(black_box(v), &locale).len());
                }
            })
        });
        c.bench_function(&format!("buffer/i128/{locale}"), |b| {
            let mut buffer = Buffer::new();
            b.iter(|| {
                for &v in &values {
                    let wide = (v as i128).wrapping_mul(-0x0123_4567_89AB_CDEF);
                    black_box(buffer.write_formatted(black_box(wide), &locale).len());
                }
            })
        });
    }
    c.bench_function("naive/u64/en", |b| {
        b.iter(|| {
            for &v in &values {
                black_box(naive(black_box(v), ",").len());
            }
        })
    });
}

/// Median-of-5 wall time for `runs` passes of `f` over the value set.
fn median_secs(mut f: impl FnMut()) -> f64 {
    let mut samples: Vec<f64> = (0..5)
        .map(|_| {
            let start = Instant::now();
            f();
            start.elapsed().as_secs_f64()
        })
        .collect();
    samples.sort_by(f64::total_cmp);
    samples[2]
}

fn ratio_gate() {
    let values = values();
    let rounds = 2000;
    let mut buffer = Buffer::new();
    let fast = median_secs(|| {
        for _ in 0..rounds {
            for &v in &values {
                black_box(buffer.write_formatted(black_box(v), &Locale::en).len());
            }
        }
    });
    let slow = median_secs(|| {
        for _ in 0..rounds {
            for &v in &values {
                black_box(naive(black_box(v), ",").len());
            }
        }
    });
    let ratio = slow / fast;
    println!("throughput gate: buffer {fast:.4}s, naive {slow:.4}s, ratio {ratio:.2}x");
    assert!(
        ratio >= 4.0,
        "u64-under-en ratio {ratio:.2}x fell below the 4x regression gate"
    );
}

criterion_group!(benches, bench_buffer);

fn main() {
    benches();
    Criterion::default().final_summary();
    ratio_gate();
}
