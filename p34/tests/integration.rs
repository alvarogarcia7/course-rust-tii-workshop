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
            one[one.len() - 1] = 1u64;
            BigUint4096::from(one)
        };

        let actual = {
            let mut one = [0; 64];
            one[one.len() - 1] = 1u64;
            let one_biguint = BigUint4096::from(one);
            let mut actual = BigUint4096::new();
            actual.sum(&one_biguint);
            actual
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one_to_two_limbs() {
        let expected = {
            let mut operand = [0; 64];
            operand[operand.len() - 1] = 1u64;
            operand[operand.len() - 2] = 1u64;
            BigUint4096::from(operand)
        };

        let actual = {
            let mut operand = [0; 64];
            operand[operand.len() - 1] = 1u64;
            operand[operand.len() - 2] = 1u64;
            let one = BigUint4096::from(operand);
            let mut actual = BigUint4096::new();
            actual.sum(&one);
            actual
        };

        assert_eq!(actual, expected);
    }
}
