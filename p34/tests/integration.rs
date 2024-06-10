pub mod integration {
    use std::ops::{Add, Mul};

    use p34::biguint::BigUint4096;
    use p34::build_biguint_max_64;

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
        let one = build_biguint_max_64!(1);

        let actual = BigUint4096::new().add(one);

        assert_eq!(actual, build_biguint_max_64!(1));
    }

    #[test]
    fn add_one_to_all_limbs() {
        let operand = BigUint4096::from([1; 64]);
        let expected = BigUint4096::from([1; 64]);

        let actual = BigUint4096::new().add(operand);

        assert_eq!(actual, expected);
    }

    #[test]
    fn add_one_to_all_limbs_with_carry() {
        let m = u64::MAX;
        let mut _operand_1 = build_biguint_max_64!(m, m, 0, 0, 0, m, 0);
        let _____operand_2 = build_biguint_max_64!(1, 0, 0, 0, 0, 1, 1);
        let ______expected = build_biguint_max_64!(0, 0, 1, 0, 0, 0, 2);

        let actual = _operand_1.add(_____operand_2);

        assert_eq!(actual, ______expected);
    }

    #[test]
    fn add_using_the_operator() {
        let m = u64::MAX;
        let mut _operand_1 = build_biguint_max_64!(m, m, 0, 0, 0, m, 0);
        let _____operand_2 = build_biguint_max_64!(1, 0, 0, 0, 0, 1, 1);
        let ______expected = build_biguint_max_64!(0, 0, 1, 0, 0, 0, 2);

        let actual = _operand_1 + _____operand_2;

        assert_eq!(actual, ______expected);
    }

    #[test]
    fn add_to_two_limbs_with_carry() {
        let expected = build_biguint_max_64!(99, 0, 1);
        let operand_1 = build_biguint_max_64!(u64::MAX, u64::MAX);
        let operand_2 = build_biguint_max_64!(100);

        let actual = operand_1.add(operand_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_one_limb() {
        let expected = build_biguint_max_64!(u64::MAX).add(build_biguint_max_64!(u64::MAX));
        let operand_1 = build_biguint_max_64!(u64::MAX);
        let operand_2 = build_biguint_max_64!(2);

        let actual = operand_1.mul(operand_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_one_limb_without_carry() {
        let operand_1 = build_biguint_max_64!(10);
        let operand_2 = build_biguint_max_64!(10);
        let expected = build_biguint_max_64!(100);

        let actual = operand_1.mul(operand_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_one_limb_with_carry() {
        let expected = build_biguint_max_64!(0, 4);
        let operand_1 = build_biguint_max_64!(0, 2);
        let operand_2 = build_biguint_max_64!(0, 2);

        let actual = operand_1.mul(operand_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_one_limb_times_10() {
        let expected = build_biguint_max_64!(u64::MAX)
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX))
            .add(build_biguint_max_64!(u64::MAX));
        let operand_1 = build_biguint_max_64!(u64::MAX);
        let operand_2 = build_biguint_max_64!(10);

        let expected_manual = build_biguint_max_64!(u64::MAX - 9, 9);

        assert_eq!(expected_manual, expected);

        let actual = operand_1.mul(operand_2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_one_limb_times_100() {
        let mut expected_sum = build_biguint_max_64!(u64::MAX);
        let limit = 100;
        for _ in 0..limit {
            expected_sum = expected_sum.add(build_biguint_max_64!(u64::MAX));
        }

        let expected_manual = build_biguint_max_64!(u64::MAX - limit, limit);

        assert_eq!(expected_manual, expected_sum);

        let operand_1 = build_biguint_max_64!(u64::MAX);
        let operand_2 = build_biguint_max_64!(limit + 1);

        let actual = operand_1.mul(operand_2);

        assert_eq!(actual, expected_sum);
    }

    #[test]
    fn sum_repeated_times() {
        let mut expected_sum = build_biguint_max_64!(u64::MAX);
        let limit = 100;
        for _ in 0..limit {
            expected_sum = expected_sum.add(build_biguint_max_64!(u64::MAX));
        }

        let expected_manual = build_biguint_max_64!(u64::MAX - limit, limit);

        assert_eq!(expected_manual, expected_sum);
    }

    #[test]
    #[should_panic]
    fn overflow_the_structure_should_panic() {
        let operand_1 = BigUint4096::from([u64::MAX; 64]);
        let operand_2 = build_biguint_max_64!(100);

        let _ = operand_1.add(operand_2);
    }
}
