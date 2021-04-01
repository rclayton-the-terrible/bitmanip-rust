use super::common::{reverse};

fn to_decimal_places_vec(num: u64) -> Vec<u8> {
    let mut places = vec![];
    let mut cur_value = num;

    while cur_value != 0 {
        let place = cur_value % 10;
        places.push(place as u8);
        cur_value = (cur_value - place) / 10;
    }

    reverse(places)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_correct_place_values() {
        assert_eq!(vec![1, 2, 3, 4, 5], to_decimal_places_vec(12345));
        assert_eq!(vec![2, 5, 1, 9], to_decimal_places_vec(2519));
    }
}
