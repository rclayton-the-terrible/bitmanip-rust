pub fn bitwise_is_odd(int: i64) -> bool {
    int & 1 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_is_odd() {
        assert_eq!(true, bitwise_is_odd(1));
        assert_eq!(true, bitwise_is_odd(255));
        assert_eq!(false, bitwise_is_odd(2));
        assert_eq!(false, bitwise_is_odd(94));
    }
}
