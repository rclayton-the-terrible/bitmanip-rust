pub fn reverse<T:Clone>(target: Vec<T>) -> Vec<T> {
    let mut copy: Vec<T> = target.clone();
    let mut reversed: Vec<T> = vec![];
    loop {
        match copy.pop() {
            None => break,
            Some(x) => reversed.push(x),
        }
    }
    reversed
}
