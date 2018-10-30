//!
//! This module contains the single trait [`IntegerSquareRoot`] and implements it for primitive
//! integer types.
//!
//! # Example
//!
//! ```
//! extern crate integer_sqrt;
//! // `use` trait to get functionality
//! use integer_sqrt::IntegerSquareRoot;
//!
//! # fn main() {
//! assert_eq!(4u8.integer_sqrt(), 2);
//! # }
//! ```
//!
//! [`IntegerSquareRoot`]: ./trait.IntegerSquareRoot.html
#![no_std]

/// A trait implementing integer square root.
pub trait IntegerSquareRoot {
    /// Find the integer square root.
    ///
    /// See [Integer_square_root on wikipedia][wiki_article] for more information (and also the
    /// source of this algorithm)
    ///
    /// # Panics
    ///
    /// For negative numbers (`i` family) this function will panic on negative input
    ///
    /// [wiki_article]: https://en.wikipedia.org/wiki/Integer_square_root
    fn integer_sqrt(&self) -> Self where Self: Sized {
        self.integer_sqrt_checked().expect("cannot calculate square root of negative number")
    }

    /// Find the integer square root, returning `None` if the number is negative (this can never
    /// happen for unsigned types).
    fn integer_sqrt_checked(&self) -> Option<Self> where Self: Sized;
}

// This could be more optimized
macro_rules! impl_isqrt {
    () => ();
    ($t:ty) => {impl_isqrt!($t,);};
    ($t:ty, $($e:tt)*) => {
        impl IntegerSquareRoot for $t {
            #[allow(unused_comparisons)]
            fn integer_sqrt_checked(&self) -> Option<Self> {
                // Hopefully this will be stripped for unsigned numbers (impossible condition)
                if *self < 0 {
                    return None
                }
                // Find greatest shift
                let mut shift = 2;
                let mut n_shifted = *self >> shift;
                // We check for n_shifted being self, since some implementations of logical
                // right shifting shift modulo the word size.
                while n_shifted != 0 && n_shifted != *self {
                    shift = shift + 2;
                    n_shifted = self.wrapping_shr(shift);
                }
                shift = shift - 2;

                // Find digits of result.
                let mut result = 0;
                loop {
                    result = result << 1;
                    let candidate_result = result + 1;
                    if candidate_result * candidate_result <= *self >> shift {
                        result = candidate_result;
                    }
                    if shift == 0 {
                        break;
                    }
                    shift = shift.saturating_sub(2);
                }

                Some(result)
            }
        }

        impl_isqrt!($($e)*);
    };
}


impl_isqrt!(usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8);


#[cfg(test)]
mod tests {
    use super::IntegerSquareRoot;
    use core::{u8, u16, u64, i8};

    #[test]
    fn u8_sqrt() {
        let tests = [
            (0u8, 0u8),
            (4, 2),
            (7, 2),
            (81, 9),
            (80, 8),
            (u8::MAX, (u8::MAX as f64).sqrt() as u8),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
    }

    #[test]
    fn i8_sqrt() {
        let tests = [
            (0i8, 0i8),
            (4, 2),
            (7, 2),
            (81, 9),
            (80, 8),
            (i8::MAX, (i8::MAX as f64).sqrt() as i8),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
    }

    #[test]
    #[should_panic]
    fn i8_sqrt_negative() {
        (-12i8).integer_sqrt();
    }

    #[test]
    fn u16_sqrt() {
        let tests = [
            (0u16, 0u16),
            (4, 2),
            (7, 2),
            (81, 9),
            (80, 8),
            (u16::MAX, (u16::MAX as f64).sqrt() as u16),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
    }

    #[test]
    fn u64_sqrt() {
        let sqrt_max = 4_294_967_295;
        let tests = [
            (0u64, 0u64),
            (4, 2),
            (7, 2),
            (81, 9),
            (80, 8),
            (u64::MAX, sqrt_max),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
        // checks to make sure we have the right number for u64::MAX.integer_sqrt()
        // we can't use the same strategy as in previous tests as f64 is now not returning the
        // correct floored integer
        assert!(sqrt_max * sqrt_max <= u64::MAX);
        // check that the next number's square overflows
        assert!((sqrt_max + 1).checked_mul(sqrt_max + 1).is_none());
    }

    #[test]
    fn u128_sqrt() {
        let sqrt_max: u128 = 18_446_744_073_709_551_615;
        let tests = [
            (0u128, 0u128),
            (4, 2),
            (7, 2),
            (81, 9),
            (80, 8),
            (u128::max_value(), sqrt_max),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
        assert!(sqrt_max * sqrt_max <= u128::max_value());
        assert!((sqrt_max + 1).checked_mul(sqrt_max + 1).is_none());
    }
}
