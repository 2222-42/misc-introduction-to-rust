pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y << 5;
        y = y & x;
    }
    y
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
