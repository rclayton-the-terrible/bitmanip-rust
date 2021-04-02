pub fn opposite_signs(a: i64, b: i64) -> bool {
    // if a or b is negative, the leading bit will be 1.
    // if positive, leading bit is 0.
    // so:
    //   1 ^ 0 = 1 (negative)
    //   0 ^ 1 = 1 (negative)
    //   0 ^ 0 = 0 (positive)
    //   1 ^ 1 = 0 (positive)
    // which is why if the signs are different on a and b,
    // the value will be < 0.
    (a ^ b) < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite_signs() {
        assert_eq!(true, opposite_signs(1, -23));
        assert_eq!(false, opposite_signs(-23, -23));
        assert_eq!(true, opposite_signs(-34, 3));
        assert_eq!(false, opposite_signs(61, 6));
        assert_eq!(true, opposite_signs(11, -23));
    }
}
