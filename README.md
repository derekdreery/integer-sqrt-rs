
# Integer square root

This module contains the single trait `IntegerSquareRoot` and implements it 
for primitive integer types.

## Example

```rust
extern crate integer_sqrt;

// `use` trait to get functionality
use integer_sqrt::IntegerSquareRoot;

assert_eq!(4u8.integer_sqrt(), 2);
```

