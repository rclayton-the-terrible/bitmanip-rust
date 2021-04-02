pub fn find_missing_number(arr: &[i64]) -> i64 {
    let mut rem: i64 = 0;
    for i in 0..(arr.len() + 1) {
        rem ^= i as i64;
    }
    for i in 0..arr.len() {
        rem ^= arr[i];
    }
    rem
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_number() {
        assert_eq!(2, find_missing_number(&[3, 0, 1]));
        assert_eq!(8, find_missing_number(&[9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }
}
