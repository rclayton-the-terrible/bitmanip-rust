pub fn bits_set(int: i64) -> u64 {
    let mut bits_set: u64 = 0;
    let mut rem: i64 = int;
    while rem > 0 {
        if rem % 2 == 1 {
            bits_set += 1;
        }
        rem = f64::floor((rem / 2) as f64) as i64
    }
    bits_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_set() {
        assert_eq!(1, bits_set(1));
        assert_eq!(1, bits_set(2));
        assert_eq!(4, bits_set(58));
        assert_eq!(5, bits_set(299));
    }
}
