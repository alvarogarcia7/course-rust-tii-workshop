#![feature(test)]

extern crate test;

#[cfg(test)]
mod benchmarks {
    use test::Bencher;

    use p22::fibonacci::{fibonacci_non_recursive, fibonacci_recursive};

    #[bench]
    fn non_recursive(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..=30 {
                fibonacci_non_recursive(std::hint::black_box(i));
            }
        });
    }

    #[bench]
    fn recursive(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..=30 {
                fibonacci_recursive(std::hint::black_box(i));
            }
        });
    }
}
