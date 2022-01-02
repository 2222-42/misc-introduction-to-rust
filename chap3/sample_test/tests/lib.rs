use sample_test::add;

#[test]
fn test_add() {
    assert!(true);
    // assert!(false, "panic! value={}", false);
    assert_eq!(0, add(0, 0));
    assert_ne!(0, add(0, 1));
    // assert_eq!(true, false, "panic! {}={} ", true, false);
}
