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
    }

    impl BigUint4096 {
        pub fn new() -> Self {
            Self::from([0; 64])
        }
    }

    impl Default for BigUint4096 {
        fn default() -> Self {
            Self::new()
        }
    }
}
