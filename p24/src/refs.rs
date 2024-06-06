// adssa
//  Write the following functions
// – f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable reference to the first element in the tuple. If flag is true, returns mutable reference to the second element in the tuple.
// – f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element in the slice (counting from zero).
// – f3: accepts slice &[u32] and number n, returns mutable reference to the n-th element from the end of the slice (counting from zero, i.e. 0 means the last element).
// – f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4 resulting slices

pub fn f1(x: &mut (u32, u32), b: bool) -> &mut u32 {
    if b {
        &mut x.1
    } else {
        &mut x.0
    }
}

pub fn f2(slice: &mut [u32], index: usize) -> &mut u32 {
    &mut slice[index]
}

pub fn f3(slice: &mut [u32], index_from_end: usize) -> &mut u32 {
    &mut slice[slice.len() - index_from_end - 1]
}

pub fn f4(slice: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let len = slice.len();
    (
        &slice[0..len / 4],
        &slice[len / 4..len / 2],
        &slice[len / 2..3 * len / 4],
        &slice[3 * len / 4..len],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f1_acccepts_a_mutable_reference_case_1() {
        let mut x = (0u32, 1u32);
        // assert_eq!(*f1(&mut x, true), 1u32);
        assert_eq!(f1(&mut x, true), &mut 1); //Better NOT to use to dereferencing
    }

    #[test]
    fn f1_acccepts_a_mutable_reference_case_2() {
        let mut x = (0u32, 1u32);
        // assert_eq!(*f1(&mut x, false), 0u32);
        assert_eq!(f1(&mut x, false), &mut 0); //Better NOT to use to dereferencing
    }

    #[test]
    fn f2_returns_first() {
        let mut input = vec![0, 1, 2, 3];
        assert_eq!(f2(&mut input, 0), &mut 0u32);
    }

    #[test]
    fn f2_returns_second() {
        let mut input = vec![0, 1, 2, 3];
        assert_eq!(f2(&mut input, 1), &mut 1u32);
    }

    #[test]
    fn f3_returns_last() {
        let mut input = vec![0, 1, 2, 3];
        assert_eq!(f3(&mut input, 0), &mut 3u32);
    }

    #[test]
    fn f3_returns_second_last() {
        let mut input = vec![0, 1, 2, 3];
        assert_eq!(f3(&mut input, 1), &mut 2u32);
    }

    #[test]
    fn f4_returns_one_each() {
        let input = vec![0, 1, 2, 3];
        let slice1: Vec<u32> = vec![0];
        assert_eq!(
            f4(&input),
            (&*slice1, &*vec![1u32], &*vec![2u32], &*vec![3u32])
        );
    }

    #[test]
    fn f4_returns_two_each() {
        let input: Vec<u32> = (0..8).collect();
        assert_eq!(
            f4(&input),
            (
                &*(0..2).collect::<Vec<_>>(),
                &*(2..4).collect::<Vec<_>>(),
                &*(4..6).collect::<Vec<_>>(),
                &*(6..8).collect::<Vec<_>>()
            )
        );
    }

    #[test]
    fn f4_returns_one_each_another_syntax() {
        let input = vec![0, 1, 2, 3];
        let slice1: Vec<u32> = vec![0];
        let res = f4(&input);
        assert_eq!(res, (&*slice1, &*vec![1u32], &*vec![2u32], &*vec![3u32]));

        // asserting individually:
        assert_eq!(res.0, &[0u32]);
    }
}
