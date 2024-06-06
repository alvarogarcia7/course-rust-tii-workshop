/// Convert Celsius degrees to Fahrenheit degrees
// Example for good documentation: https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#944
pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    // Careful with https://rust-lang.github.io/rust-clippy/master/index.html#/identity_op
    ((celsius * 9) / 5) + 32
}

/// Convert Fahrenheit degrees to Celsius degrees
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    // Careful with https://rust-lang.github.io/rust-clippy/master/index.html#/identity_op
    (5 * (fahrenheit - 32)) / 9
}

#[cfg(test)]
mod tests {
    use crate::celsius_fahrenheit_converter::{celsius2fahrenheit, fahrenheit2celsius};

    #[test]
    fn loop_from_celsius() {
        let same_celsius_value = -5;
        assert_eq!(
            fahrenheit2celsius(celsius2fahrenheit(same_celsius_value)),
            same_celsius_value
        );
    }

    #[test]
    fn loop_from_farenheit() {
        let same_fahrenheit_value = 212;
        assert_eq!(
            celsius2fahrenheit(fahrenheit2celsius(same_fahrenheit_value)),
            same_fahrenheit_value
        );
    }

    #[test]
    fn kat() {
        // Source: https://fr.farnell.com/en-FR/convertisseur-temperature
        let equivalences_fahrenheit_and_celsius = [(23, -5), (194, 90), (212, 100), (392, 200)];
        for equivalence in equivalences_fahrenheit_and_celsius {
            let actual_celsius = fahrenheit2celsius(equivalence.0);
            assert_eq!(
                equivalence.1, actual_celsius,
                "error: fahrenheit {} is not celsius {}. actual: {}",
                equivalence.0, equivalence.1, actual_celsius
            );

            let actual_fahrenheit = celsius2fahrenheit(equivalence.1);
            assert_eq!(
                equivalence.1, actual_celsius,
                "error: celsius {} is not fahrenheit {}. actual: {}",
                equivalence.1, equivalence.0, actual_fahrenheit
            );

            assert_eq!(
                celsius2fahrenheit(fahrenheit2celsius(equivalence.0)),
                equivalence.0
            );
            assert_eq!(
                fahrenheit2celsius(celsius2fahrenheit(equivalence.1)),
                equivalence.1
            );
        }
    }
}
