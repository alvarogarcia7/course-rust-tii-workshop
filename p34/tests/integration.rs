pub mod integration {
    use p34::biguint::BigUintGeneric;
    use p34::build_biguint4096;

    #[test]
    fn are_comparable() {
        let expected = BigUintGeneric::<64>::new();

        let actual = BigUintGeneric::<64>::new();

        assert_eq!(actual, expected);
    }

    #[test]
    fn default_to_zero() {
        let expected = BigUintGeneric::<64>::new();

        let actual = BigUintGeneric::<64>::from([0; 64]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one() {
        let one = build_biguint4096!(1);

        let actual = {
            let mut actual = BigUintGeneric::<64>::new();
            actual.sum(&one);
            actual
        };

        assert_eq!(actual, build_biguint4096!(1));
    }

    #[test]
    fn add_one_to_all_limbs() {
        let expected = BigUintGeneric::<64>::from([1; 64]);
        let operand = BigUintGeneric::<64>::from([1; 64]);

        let actual = {
            let mut actual = BigUintGeneric::<64>::new();
            actual.sum(&operand);
            actual
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one_to_all_limbs_with_carry() {
        let m = u64::MAX;
        let mut _operand_1 = build_biguint4096!(m, m, 0, 0, 0, m, 0);
        let _____operand_2 = build_biguint4096!(1, 0, 0, 0, 0, 1, 1);
        let ______expected = build_biguint4096!(0, 0, 1, 0, 0, 0, 2);

        _operand_1.sum(&_____operand_2);

        assert_eq!(_operand_1, ______expected);
    }

    #[test]
    fn add_to_two_limbs_with_carry() {
        let expected = build_biguint4096!(99, 0, 1);
        let mut operand_1 = build_biguint4096!(u64::MAX, u64::MAX);
        let operand_2 = build_biguint4096!(100);

        operand_1.sum(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn multiply_one_limb() {
        let mut expected = build_biguint4096!(u64::MAX);
        expected.sum(&build_biguint4096!(u64::MAX));
        let mut operand_1 = build_biguint4096!(u64::MAX);
        let operand_2 = build_biguint4096!(2);

        operand_1.multiply(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn multiply_one_limb_without_carry() {
        let mut operand_1 = build_biguint4096!(10);
        let operand_2 = build_biguint4096!(10);
        let expected = build_biguint4096!(100);

        operand_1.multiply(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn multiply_one_limb_with_carry() {
        let expected = build_biguint4096!(0, 4);
        let mut operand_1 = build_biguint4096!(0, 2);
        let operand_2 = build_biguint4096!(0, 2);

        operand_1.multiply(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn multiply_one_limb_times_10() {
        let mut expected = build_biguint4096!(u64::MAX);

        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        expected.sum(&build_biguint4096!(u64::MAX));
        let mut operand_1 = build_biguint4096!(u64::MAX);
        let operand_2 = build_biguint4096!(10);

        let expected_manual = build_biguint4096!(u64::MAX - 9, 9);

        assert_eq!(expected_manual, expected);

        operand_1.multiply(&operand_2);

        assert_eq!(operand_1, expected);
    }

    #[test]
    fn multiply_one_limb_times_100() {
        let mut expected_sum = build_biguint4096!(u64::MAX);
        let limit = 100;
        for _ in 0..limit {
            expected_sum.sum(&build_biguint4096!(u64::MAX));
        }

        let expected_manual = build_biguint4096!(u64::MAX - limit, limit);

        assert_eq!(expected_manual, expected_sum);

        let mut operand_1 = build_biguint4096!(u64::MAX);
        let operand_2 = build_biguint4096!(limit + 1);

        operand_1.multiply(&operand_2);

        assert_eq!(operand_1, expected_sum);
    }

    #[test]
    fn sum_repeated_times() {
        let mut expected_sum = build_biguint4096!(u64::MAX);
        let limit = 100;
        for _ in 0..limit {
            expected_sum.sum(&build_biguint4096!(u64::MAX));
        }

        let expected_manual = build_biguint4096!(u64::MAX - limit, limit);

        assert_eq!(expected_manual, expected_sum);
    }

    #[test]
    #[should_panic]
    fn overflow_the_structure_should_panic() {
        let mut operand_1 = BigUintGeneric::<64>::from([u64::MAX; 64]);
        let operand_2 = build_biguint4096!(100);

        operand_1.sum(&operand_2);
    }
}
