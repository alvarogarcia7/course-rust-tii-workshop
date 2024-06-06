#![feature(test)]

extern crate test;

mod non_recursive {
    use p22::fibonacci::fibonacci_non_recursive;
    use test::Bencher;
    #[bench]
    fn case_10(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_non_recursive(std::hint::black_box(10));
        });
    }

    #[bench]
    fn case_20(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_non_recursive(std::hint::black_box(20));
        });
    }

    #[bench]
    fn case_30(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_non_recursive(std::hint::black_box(30));
        });
    }
}

mod recursive {
    use p22::fibonacci::fibonacci_recursive;
    use test::Bencher;
    #[bench]
    fn case_10(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_recursive(std::hint::black_box(10));
        });
    }

    #[bench]
    fn case_20(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_recursive(std::hint::black_box(20));
        });
    }

    #[bench]
    fn case_30(b: &mut Bencher) {
        b.iter(|| {
            fibonacci_recursive(std::hint::black_box(30));
        });
    }
}
