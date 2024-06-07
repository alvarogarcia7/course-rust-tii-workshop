// extern crate test;

pub mod tests {
    use std::collections::HashMap;

    use p44::hashmap;

    #[test]
    pub fn empty_macro() {
        let expected = HashMap::<u64, u64>::new();

        let actual = hashmap!();

        assert_eq!(expected, actual);
    }
}
