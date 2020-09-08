#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

extern crate integer_sqrt;
use integer_sqrt::IntegerSquareRoot;

// Use f64::sqrt to compute the integer sqrt
fn isqrt_via_f64(n: u64) -> u64 {
    let cand = (n as f64).sqrt() as u64;
    // Rounding can cause off-by-one errors
    if let Some(prod) = cand.checked_mul(cand) {
        if prod <= n {
            return cand;
        }
    }
    cand - 1
}

#[bench]
fn isqrt_small(b: &mut Bencher) {
    let small = 63u64;
    b.iter(|| {
        let n = black_box(small);
        assert_eq!(n.integer_sqrt_checked(), Some(7));
    })
}

#[bench]
fn isqrt_med(b: &mut Bencher) {
    let med = 10_000_000_000u64; // 10^10
    b.iter(|| {
        let n = black_box(med);
        assert_eq!(n.integer_sqrt_checked(), Some(100_000)); // 10^5
    })
}

#[bench]
fn isqrt_large(b: &mut Bencher) {
    let large = u64::MAX;
    b.iter(|| {
        let n = black_box(large);
        assert_eq!(n.integer_sqrt_checked(), Some((1u64 << 32) - 1));
    })
}

#[bench]
fn isqrt_f64_small(b: &mut Bencher) {
    let small = 63u64;
    b.iter(|| {
        let n = black_box(small);
        assert_eq!(isqrt_via_f64(n), 7);
    })
}

#[bench]
fn isqrt_f64_med(b: &mut Bencher) {
    let med = 10_000_000_000u64; // 10^10
    b.iter(|| {
        let n = black_box(med);
        assert_eq!(isqrt_via_f64(n), 100_000); // 10^5
    })
}

#[bench]
fn isqrt_f64_large(b: &mut Bencher) {
    let large = u64::MAX;
    b.iter(|| {
        let n = black_box(large);
        assert_eq!(isqrt_via_f64(n), (1u64 << 32) - 1);
    })
}
