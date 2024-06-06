fn fibonacci(_p0: u32) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_0() {
        assert_eq!(0, fibonacci(0))
    }
}
