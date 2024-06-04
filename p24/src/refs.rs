// adssa
//  Write the following functions
// – f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable reference to the first element in the tuple. If flag is true, returns mutable reference to the second element in the tuple.
// – f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element in the slice (counting from zero).
// – f3: accepts slice &[u32] and number n, returns mutable reference to the n-th element from the end of the slice (counting from zero, i.e. 0 means the last element).
// – f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4 resulting slices

fn f1<'a>(x: &'a mut (u32, u32), b: bool) -> &'a mut u32 {
    return if b {
        &mut x.1
    } else {
        &mut x.0
    };
}

fn f2(mut slice: &mut [u32], index: usize) -> &mut u32 {
    &mut slice[index]
}

fn f3(slice: &mut [u32], index_from_end: usize) -> &mut u32 {
    &mut slice[slice.len() - index_from_end - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f1_acccepts_a_mutable_reference__case_1() {
        let mut x = (0u32, 1u32);
        // assert_eq!(*f1(&mut x, true), 1u32);
        assert_eq!(f1(&mut x, true), &mut 1); //Better NOT to use to dereferencing
    }

    #[test]
    fn f1_acccepts_a_mutable_reference__case_2() {
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
}