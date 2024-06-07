pub mod integration {
    use p34::biguint::BigUint4096;

    #[test]
    fn are_comparable() {
        let expected = BigUint4096::new();

        let actual = BigUint4096::new();

        assert_eq!(actual, expected);
    }

    #[test]
    fn default_to_zero() {
        let expected = BigUint4096::new();

        let actual = BigUint4096::from([0; 64]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one() {
        let expected = {
            let mut one = [0; 64];
            one[0] = 1u64;
            BigUint4096::from(one)
        };

        let actual = {
            let mut one = [0; 64];
            one[0] = 1u64;
            let one_biguint = BigUint4096::from(one);
            let mut actual = BigUint4096::new();
            actual.sum(&one_biguint);
            actual
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one_to_all_limbs() {
        let expected = { BigUint4096::from([1; 64]) };

        let actual = {
            let operand = BigUint4096::from([1; 64]);
            let mut actual = BigUint4096::new();
            actual.sum(&operand);
            actual
        };

        assert_eq!(actual, expected);
    }
}
