pub fn min_flips(a: i64, b: i64, c:i64) -> i64 {
    let mut flips: i64 = 0;

    for i in 0..32 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        let bit_c = (c >> i) & 1;

        if (bit_a | bit_b) != bit_c {
            if bit_c == 0 {
                let both_need_to_be_flipped = bit_a == 1 && bit_b == 1;
                flips += if both_need_to_be_flipped { 2 } else { 1 };
            } else {
                flips += 1;
            }
        }
    }
    flips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_flips() {
        assert_eq!(3, min_flips(2, 6, 5));
    }
}
