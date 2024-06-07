pub mod biguint {
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
        }
    }

    impl Default for BigUint4096 {
        fn default() -> Self {
            Self::new()
        }
    }
}
