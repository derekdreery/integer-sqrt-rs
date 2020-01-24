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
    fn integer_sqrt(&self) -> Self
    where
        Self: Sized,
    {
        self.integer_sqrt_checked()
            .expect("cannot calculate square root of negative number")
    }

    /// Find the integer square root, returning `None` if the number is negative (this can never
    /// happen for unsigned types).
    fn integer_sqrt_checked(&self) -> Option<Self>
    where
        Self: Sized;
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
                    let candidate_result: $t = result + 1;
                    if let Some(cr_square) = candidate_result.checked_mul(candidate_result) {
                        if cr_square <= *self >> shift {
                            result = candidate_result;
                        }
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
    use core::{i8, u16, u64, u8};

    macro_rules! gen_tests {
        ($($type:ty => $fn_name:ident),*) => {
            $(
                #[test]
                fn $fn_name() {
                    let newton_raphson = |val, square| 0.5 * (val + (square / val as $type) as f64);
                    let max_sqrt = {
                        let square = <$type>::max_value();
                        let mut value = (square as f64).sqrt();
                        for _ in 0..2 {
                            value = newton_raphson(value, square);
                        }
                        let mut value = value as $type;
                        // make sure we are below the max value (this is how integer square
                        // root works)
                        if value.checked_mul(value).is_none() {
                            value -= 1;
                        }
                        value
                    };
                    let tests: [($type, $type); 9] = [
                        (0, 0),
                        (1, 1),
                        (2, 1),
                        (3, 1),
                        (4, 2),
                        (81, 9),
                        (80, 8),
                        (<$type>::max_value(), max_sqrt),
                        (<$type>::max_value() - 1, max_sqrt),
                    ];
                    for &(in_, out) in tests.iter() {
                        assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
                    }
                }
            )*
        };
    }

    gen_tests! {
        i8 => i8_test,
        u8 => u8_test,
        i16 => i16_test,
        u16 => u16_test,
        i32 => i32_test,
        u32 => u32_test,
        i64 => i64_test,
        u64 => u64_test,
        u128 => u128_test,
        isize => isize_test,
        usize => usize_test
    }

    #[test]
    fn i128_test() {
        let tests: [(i128, i128); 8] = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (81, 9),
            (80, 8),
            (i128::max_value(), 13_043_817_825_332_782_212),
        ];
        for &(in_, out) in tests.iter() {
            assert_eq!(in_.integer_sqrt(), out, "in {}", in_);
        }
    }
}
