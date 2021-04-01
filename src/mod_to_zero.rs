fn reverse(target: Vec<u8>) -> Vec<u8> {
    let mut copy: Vec<u8> = target.clone();
    let mut reversed: Vec<u8> = vec![];
    loop {
        match copy.pop() {
            None => break,
            Some(x) => reversed.push(x),
        }
    }
    reversed
}

fn get_place_values(num: u64) -> Vec<u8> {
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
        assert_eq!(vec![1, 2, 3, 4, 5], get_place_values(12345));
        assert_eq!(vec![2, 5, 1, 9], get_place_values(2519));
    }
}
