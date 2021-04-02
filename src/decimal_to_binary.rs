use super::common::{reverse};

pub fn division_method(int: i64) -> Vec<u8> {
    let mut places:Vec<u8> = vec![];
    let mut quotient:i64 = int;
    while quotient != 0 && quotient != 1 {
        let rem = quotient % 2;
        quotient = quotient / 2;
        places.push(rem as u8);
    }
    places.push(quotient as u8);
    reverse(places)
}

pub fn bitshift_method(int: i64) -> Vec<u8> {
    let mut places:Vec<u8> = vec![];
    let mut rem:i64 = int;
    while rem != 0 {
        let place = rem & 1;
        rem >>= 1;
        places.push(place as u8);
    }
    reverse(places)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_method() {
        assert_eq!(vec![1, 0, 0, 0], division_method(8));
        assert_eq!(vec![1, 0, 0, 1], division_method(9));
        assert_eq!(vec![1, 0, 1, 0], division_method(10));
        assert_eq!(vec![1, 0, 1, 0, 1], division_method(21));
    }

    #[test]
    fn test_bitshift_method() {
        assert_eq!(vec![1, 0, 0, 0], bitshift_method(8));
        assert_eq!(vec![1, 0, 0, 1], bitshift_method(9));
        assert_eq!(vec![1, 0, 1, 0], bitshift_method(10));
        assert_eq!(vec![1, 0, 1, 0, 1], bitshift_method(21));
    }
}
