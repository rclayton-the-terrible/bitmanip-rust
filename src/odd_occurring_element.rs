fn odd_occurring_element(arr: &[i64]) -> i64 {
    let mut odd_num: i64 = 0;
    for i in 0..arr.len() {
        odd_num ^= arr[i];
    }
    odd_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_occurring_element() {
        assert_eq!(5, odd_occurring_element(&[2, 3, 2, 3, 5]));
        assert_eq!(3, odd_occurring_element(&[4, 3, 3, 4, 4, 4, 5, 3, 5]));
    }
}
