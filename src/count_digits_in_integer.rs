fn division_method(int: i64) -> u64 {
    let mut num_digits: u64 = 0;
    let mut dec_value: i64 = int;
    while dec_value >= 1 {
        num_digits += 1;
        dec_value = dec_value / 10 as i64;
    }
    num_digits
}

fn log10_method(int: i64) -> u64 {
    if int == 0 {
        return 1;
    }
    ((int as f64).log10().floor() + 1_f64) as u64
}

fn recursive_method(int: i64) -> u64 {
    if int == 0 {
        return 0
    }
    recursive_method(((int / 10) as f64).floor() as i64) + 1
}

fn to_string_method(int: i64) -> u64 {
    int.to_string().len() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_division_method() {
        assert_eq!(4, division_method(1234));
        assert_eq!(10, division_method(9812376542));
    }

    #[test]
    fn test_log10_method() {
        assert_eq!(4, log10_method(1234));
        assert_eq!(10, log10_method(9812376542));
    }

    #[test]
    fn test_recursive_method() {
        assert_eq!(4, recursive_method(1234));
        assert_eq!(10, recursive_method(9812376542));
    }

    #[test]
    fn test_to_string_method() {
        assert_eq!(4, to_string_method(1234));
        assert_eq!(10, to_string_method(9812376542));
    }
}
