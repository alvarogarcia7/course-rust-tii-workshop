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
}
