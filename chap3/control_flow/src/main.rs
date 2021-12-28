struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; // 出力する型の紐付け

    // impl next()
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

fn main() {
    let array = [0, 1, 2, 3, 4, 5];
    for element in &array {
        println!("element: {}", element)
    }

    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("count: {}", num);
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
