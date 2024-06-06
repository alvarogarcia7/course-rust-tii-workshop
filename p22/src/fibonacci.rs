fn fibonacci(n: u32) -> u64 {
    n as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_0() {
        assert_eq!(0, fibonacci(0))
    }

    #[test]
    fn base_1() {
        assert_eq!(1, fibonacci(1))
    }
}
