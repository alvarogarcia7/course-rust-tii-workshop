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
        let mut carry = 0;
        for i in 0..self.value.len() {
            carry = self.sum_one_limb(i, another, carry)
        }
        if carry != 0 {
            panic!("You are overflowing the structure")
        }
    }

    fn sum_two_numbers(a: u64, b: u64) -> (u64, u64) {
        match a.checked_add(b) {
            None => {
                // 7 + 4 = 11
                // MAX = 9 (one digit)
                // 11 % 10 = 1

                // MAX - 7 = 2
                // 4 - 2 - 1 = 1 (one digit, no overflows)
                let x = u64::MAX - a;
                let y = b - x - 1;
                (y, 1u64)
            }
            Some(non_overflow) => (non_overflow, 0),
        }
    }
    fn sum_one_limb(&mut self, i: usize, another: &Self, carry: u64) -> u64 {
        let (x, carry1) = Self::sum_two_numbers(self.value[i], another.value[i]);
        let (y, carry2) = Self::sum_two_numbers(x, carry);

        self.value[i] = y;

        carry2 + carry1
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
    // AGB: Don't do this because we allow two ways of creating from an array - NO
    // AGB: Don't do this because we force the user to specify 1u64 as the literal - NO
    // Define all values using the array syntax
    // ([$value:expr; $len:expr]) => {
    //     {
    //         BigUint4096::from([$value; $len])
    //     }
    // };
}
