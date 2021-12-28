fn main() {
    let array = [0, 1, 2, 3, 4, 5];
    for element in &array {
        println!("element: {}", element)
    }

    'main: loop {
        println!("main loop start");

        loop {
            println!("sub loop start");

            break 'main;
        }
        println!("main loop end"); // unreachable statement
    }
}
