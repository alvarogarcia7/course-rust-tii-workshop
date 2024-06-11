2024-06-10 17:18:29 AGB

* Investigate RustFmt::skip https://stackoverflow.com/questions/67288537/how-can-i-switch-off-rustfmt-for-a-region-of-code-instead-of-a-single-item

```rust
#[rustfmt::skip]
mod unformatted {
    pub fn add(a : i32, b : i32) -> i32 { a + b }
    pub fn sub(a : i32, b : i32) -> i32 { a - b }
}

use unformatted::*;

fn main() {
    dbg!(add(2, 3));
}
```

