fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_non_recursive(n: u32) -> u64 {
    match n {
        0 => return 0,
        1 => return 1,
        2 => return 1,
        _ => None::<String>,
    };
    assert!(n >= 2);
    let mut past_2 = fibonacci_non_recursive(0);
    let mut past = fibonacci_non_recursive(1);
    let mut number = fibonacci_non_recursive(2);
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
mod tests_recursive {
    use super::*;

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

#[cfg(test)]
mod tests_non_recursive {
    use super::*;

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

#[cfg(test)]
mod benchmarks {
    use test::Bencher;

    use super::*;

    #[bench]
    fn recursive(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..=30 {
                fibonacci_recursive(i);
            }
            std::hint::black_box(());
        });
    }

    #[bench]
    fn non_recursive(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..=30 {
                fibonacci_non_recursive(i);
            }
            std::hint::black_box(());
        });
    }
}
