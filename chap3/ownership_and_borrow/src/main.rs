struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
}

fn main() {
    let mut x = 5;
    let _y = &x;
    let z = &mut x;
    // dbg!(_y);
    dbg!(z);
    dbg!(x);

    {
        let d = Droppable;
    }
    println!("The Droppable should be released at the end of block.");
}
