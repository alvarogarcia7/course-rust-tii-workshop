// Implement in lib.rs a simplified BigUint4096 type, i.e. unsigned 4096 bit integer
// ● Under the hood use array of u64s
// ● Browse the standard library docs for potentially useful methods:
// https://doc.rust-lang.org/stable/std/primitive.u64.html
// ● Implement basic arithmetic traits
// ● Implement conversion methods from hex-encoded &str and to hex-
// encoded String (be careful about endianness)
// ● Add integration tests for the type

/// BigUint64
/// Note: the Least Significant Bit (LSB) is stored on position 0
#[derive(Debug, PartialEq)]
pub struct BigUint4096 {
    value: [u64; 64],
}

impl BigUint4096 {
    pub fn from(initial_value: [u64; 64]) -> Self {
        Self {
            value: initial_value,
        }
    }
    pub fn new() -> Self {
        Self::from([0; 64])
    }

    pub fn sum(&mut self, another: &Self) {
        for i in 0..self.value.len() {
            self.value[i] += another.value[i];
        }
    }
}

impl Default for BigUint4096 {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! build_biguint4096 {
    () => {
        BigUint4096::new();
    };
    // Define the first few values, everything else is zero
    ($($value:expr $(,)?)+) => {
        {
            let mut v = [0;64];
            let mut i = 0;
            $(
            v[i] = $value;
            i += 1;
            )*
            BigUint4096::from(v)
        }
    };
}
