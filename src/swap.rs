pub fn swap(a: &mut i64, b: &mut i64) {
    *a = *a ^ *b;
    *b = *b ^ *a;
    *a = *a ^ *b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut a1: i64 = 23;
        let mut b1: i64 = 32;
        swap(&mut a1, &mut b1);
        assert_eq!(a1, 32);
        assert_eq!(b1, 23);
    }
}
