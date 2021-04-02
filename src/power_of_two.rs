use super::bits_set::kernighans_bit_set;

pub fn bitwise_is_power_of_two(int: i64) -> bool {
    kernighans_bit_set(int) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_is_power_of_two() {
        assert_eq!(true, bitwise_is_power_of_two(2));
        assert_eq!(false, bitwise_is_power_of_two(3));
        assert_eq!(false, bitwise_is_power_of_two(9));
        assert_eq!(true, bitwise_is_power_of_two(16));
        assert_eq!(true, bitwise_is_power_of_two(32));
        assert_eq!(false, bitwise_is_power_of_two(95));
        assert_eq!(true, bitwise_is_power_of_two(1024));
    }
}
