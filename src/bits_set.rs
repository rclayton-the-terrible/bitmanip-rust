pub fn division_bits_set(int: i64) -> u64 {
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

pub fn kernighans_bit_set(int: i64) -> u64 {
    let mut bits_set: u64 = 0;
    let mut rem: i64 = int;
    while rem > 0 {
        rem &= rem - 1;
        bits_set += 1;
    }
    bits_set
}

pub fn is_kth_bit_set(int: i64, k: i64) -> bool {
    if int == 0 {
        return false;
    }
    // this sets "1" as the kth value of a binary array (i64)
    // that we can use with bitwise & to see if the bit is set.
    let check_bits = 1 << (k - 1);
    // if the bit is set on both numbers, the value will be
    // the value of check_bits.  If it's zero, there are no
    // matching bits.
    (int & check_bits) != 0
}



pub fn first_set_bit(int: i64) -> Option<i64> {
    if int == 0 {
        return None
    }
    // Interestingly, JavaScript ignores "1 << (0 - 1)"
    // but this does not work in Rust (causes "attempt to shift left with overflow").
    for i in 1..64 {
        let check_bits = 1 << (i - 1);
        if (int & check_bits) != 0 {
            return Some(i as i64);
        }
    }
    None
}

pub fn first_set_bit_from_right(int: i64) -> Option<i64> {
    if int == 0 {
        return None
    }
    for i in 1..64 {
        if ((int >> i - 1) & 1) == 1 {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_bits_set() {
        assert_eq!(1, division_bits_set(1));
        assert_eq!(1, division_bits_set(2));
        assert_eq!(4, division_bits_set(58));
        assert_eq!(5, division_bits_set(299));
    }

    #[test]
    fn test_kernighans_bits_set() {
        assert_eq!(1, kernighans_bit_set(1));
        assert_eq!(1, kernighans_bit_set(2));
        assert_eq!(4, kernighans_bit_set(58));
        assert_eq!(5, kernighans_bit_set(299));
    }

    #[test]
    fn test_is_kth_bit_set() {
        assert_eq!(true, is_kth_bit_set(5, 3));
        assert_eq!(true, is_kth_bit_set(10, 2));
        assert_eq!(false, is_kth_bit_set(10, 1));
    }

    #[test]
    fn test_first_set_bit() {
        assert_eq!(Some(2), first_set_bit(18));
        assert_eq!(Some(1), first_set_bit(5));
        assert_eq!(Some(6), first_set_bit(32));
        assert_eq!(None, first_set_bit(0));
    }

    #[test]
    fn test_first_set_bit_from_right() {
        assert_eq!(Some(2), first_set_bit_from_right(18));
        assert_eq!(Some(1), first_set_bit_from_right(5));
        assert_eq!(Some(6), first_set_bit_from_right(32));
        assert_eq!(None, first_set_bit(0));
    }
}
