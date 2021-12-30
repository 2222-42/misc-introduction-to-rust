fn main() {
    let mut x = 5;
    let _y = &x;
    let z = &mut x;
    // dbg!(_y);
    dbg!(z);
    dbg!(x);
}
