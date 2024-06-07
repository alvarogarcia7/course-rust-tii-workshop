// Implement in lib.rs a simplified BigUint4096 type, i.e. unsigned 4096 bit integer
// ● Under the hood use array of u64s
// ● Browse the standard library docs for potentially useful methods:
// https://doc.rust-lang.org/stable/std/primitive.u64.html
// ● Implement basic arithmetic traits
// ● Implement conversion methods from hex-encoded &str and to hex-
// encoded String (be careful about endianness)
// ● Add integration tests for the type

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
        self.value[63] += another.value[63];
        self.value[62] += another.value[62];
    }
}

impl Default for BigUint4096 {
    fn default() -> Self {
        Self::new()
    }
}
