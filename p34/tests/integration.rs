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
        let expected = BigUint4096::from([1; 64]);
        let operand = BigUint4096::from([1; 64]);

        let actual = {
            let mut actual = BigUint4096::new();
            actual.sum(&operand);
            actual
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one_to_all_limbs_with_carry() {
        let expected = build_biguint4096!(0, 0, 1);
        let mut operand_1 = build_biguint4096!(u64::MAX, u64::MAX);
        let operand_2 = build_biguint4096!(1);

        operand_1.sum(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn add_to_two_limbs_with_carry() {
        let expected = build_biguint4096!(99, 0, 1);
        let mut operand_1 = build_biguint4096!(u64::MAX, u64::MAX);
        let operand_2 = build_biguint4096!(100);

        operand_1.sum(&operand_2);

        assert_eq!(operand_1, expected);
    }
}
