2024-06-07 12:57:04 AGB

# Generic in macros

Possible enhancement:

```rust
hashmap::<u64, u64> ! (
1 => 2
);
```

syntax to be defined.

# Alternative case matching

Imagine we need a specific version when 3 or more elements:

in a POSIX BRE regex, we could write

```rust
($($key:expr => $value:expr $(,)?)\{3,\}) => {
```

in a POSIX ERE regex, we could write

```rust
($($key:expr => $value:expr $(,)?){3,}) => {
```

Is there any alternative?
