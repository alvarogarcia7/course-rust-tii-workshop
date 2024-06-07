pub mod tests {
    use std::collections::HashMap;

    use p44::hashmap;

    #[test]
    pub fn empty_macro() {
        let expected = HashMap::<u64, bool>::new();

        let actual = hashmap!();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn one_element() {
        // Alternative syntax with mutation
        // let mut expected = HashMap::<u64, bool>::new();
        // expected.insert(42, true);

        let expected = HashMap::<u64, bool>::from([(42, true)]);

        let actual = hashmap!(
            42 => true
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn one_element_with_trailing_comma() {
        let expected = HashMap::<u64, bool>::from([(42, true)]);

        let actual = hashmap!(
            42 => true,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn two_elements() {
        let expected = HashMap::<u64, bool>::from([(42, true), (43, true)]);

        let actual = hashmap!(
            42 => true,
            43 => true,
        );

        assert_eq!(expected, actual);
    }
}
