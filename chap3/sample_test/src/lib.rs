/// This function adds 2 numbers.
/// # Example
/// ```
/// use sample_test::add;
/// add(1,2);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
#[should_panic]
fn test_panic() {
    assert_eq!(true, false, "expected panic! {}={} ", true, false);
}

#[test]
#[ignore = "too slow"]
fn test_add_ignored() {
    assert_eq!(-2, add(-1, -1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
