pub mod integration {
    use p34::biguint::BigUint4096;
    use p34::build_biguint4096;

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
        let one = build_biguint4096!(1);

        let actual = {
            let mut actual = BigUint4096::new();
            actual.sum(&one);
            actual
        };

        assert_eq!(actual, build_biguint4096!(1));
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
