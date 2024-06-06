//! Fibonacci series - Recursive implementation
//! # Examples
//! ```
//! use p22::fibonacci::fibonacci_recursive as fib;
//! assert_eq!(0, fib(0));
//! assert_eq!(1, fib(1));
//! assert_eq!(1, fib(2));
//! ```
pub fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

const PRECOMPUTED_VALUES: [u64; 3] = [0, 1, 1];

/// Fibonacci series - Non Recursive implementation
/// # Examples
/// ```
/// use p22::fibonacci::fibonacci_non_recursive as fib;
/// assert_eq!(0, fib(0));
/// assert_eq!(1, fib(1));
/// assert_eq!(1, fib(2));
/// ```
pub fn fibonacci_non_recursive(n: u32) -> u64 {
    match n {
        0 => return PRECOMPUTED_VALUES[n as usize],
        1 => return PRECOMPUTED_VALUES[n as usize],
        2 => return PRECOMPUTED_VALUES[n as usize],
        _ => None::<String>,
    };
    assert!(n >= 2);
    let mut past_2 = PRECOMPUTED_VALUES[0];
    let mut past = PRECOMPUTED_VALUES[1];
    let mut number = PRECOMPUTED_VALUES[2];
    // Range inclusive syntax ..= - source: https://bencher.dev/learn/benchmarking/rust/libtest-bench/
    for _ in 2u64..=n as u64 {
        number = past + past_2;

        // Shift the memory registers to the past
        past_2 = past;
        past = number;
    }
    number
}

// TODO: is this configuration useful?
#[cfg(test)]
// Source: https://oeis.org/A000045
static FIBONACCI_NUMBERS: [u64; 41] = [
    0,
    1,
    1,
    2,
    3,
    5,
    8,
    13,
    21,
    34,
    55,
    89,
    144,
    233,
    377,
    610,
    987,
    1597,
    2584,
    4181,
    6765,
    10946,
    17711,
    28657,
    46368,
    75025,
    121393,
    196418,
    317811,
    514229,
    832040,
    1346269,
    2178309,
    3524578,
    5702887,
    9227465,
    14930352,
    24157817,
    39088169,
    63245986,
    102334155u64,
];

#[cfg(test)]
mod tests {
    mod recursive {
        use super::super::*;

        #[test]
        fn base_0() {
            assert_eq!(0, fibonacci_recursive(0))
        }

        #[test]
        fn base_1() {
            assert_eq!(1, fibonacci_recursive(1))
        }

        #[test]
        fn kat() {
            for (index, &expected_fibonacci_number) in FIBONACCI_NUMBERS[0..10].iter().enumerate() {
                let actual_fibonacci_number = fibonacci_recursive(index as u32);
                assert_eq!(
                    expected_fibonacci_number, actual_fibonacci_number,
                    "error: fibonacci({}) is not {}. actual: {}",
                    index, expected_fibonacci_number, actual_fibonacci_number
                );
            }
        }
    }

    mod non_recursive {
        use super::super::*;

        #[test]
        fn base_0() {
            assert_eq!(0, fibonacci_non_recursive(0))
        }

        #[test]
        fn base_1() {
            assert_eq!(1, fibonacci_non_recursive(1))
        }

        #[test]
        fn kat() {
            for (index, &expected_fibonacci_number) in FIBONACCI_NUMBERS.iter().enumerate() {
                let actual_fibonacci_number = fibonacci_non_recursive(index as u32);
                assert_eq!(
                    expected_fibonacci_number, actual_fibonacci_number,
                    "error: fibonacci({}) is not {}. actual: {}",
                    index, expected_fibonacci_number, actual_fibonacci_number
                );
            }
        }
    }
}
