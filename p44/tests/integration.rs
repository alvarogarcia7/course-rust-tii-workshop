pub mod tests_basic_version {
    use std::collections::HashMap;

    use p44::hashmap;

    #[test]
    pub fn empty_macro() {
        let expected = HashMap::<u64, bool>::new();

        let actual: HashMap<u64, bool> = hashmap!();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn one_element() {
        // Alternative syntax with mutation
        // let mut expected = HashMap::<u64, bool>::new();
        // expected.insert(42, true);

        let expected = HashMap::<u64, bool>::from([(42, true)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn one_element_with_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn two_elements_with_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true), (43, true)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true,
            43 => true,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn two_elements_without_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true), (43, true)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true,
            43 => true
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn n_elements_with_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true), (43, true), (44, false)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true,
            43 => true,
            44 => false,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn n_elements_without_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true), (43, true), (44, false)]);

        let actual: HashMap<u64, bool> = hashmap!(
            42 => true,
            43 => true,
            44 => false
        );

        assert_eq!(expected, actual);
    }
}

pub mod tests_poorgeneric_version {
    use std::collections::HashMap;

    use p44::hashmap_poorgeneric;

    #[test]
    pub fn zero_elements() {
        let expected = HashMap::<u64, u64>::new();

        let actual: HashMap<u64, u64> = hashmap_poorgeneric!(<u64, u64>
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn n_elements_without_trailing_comma() {
        let expected = HashMap::<u64, u64>::from([(42, 42), (43, 43), (44, 44)]);

        let actual: HashMap<u64, u64> = hashmap_poorgeneric!(<u64, u64>,
            42 => 42,
            43 => 43,
            44 => 44,
        );

        assert_eq!(expected, actual);
    }
}
