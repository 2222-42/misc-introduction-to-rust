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

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }

    fn take_age(&mut self) -> &Self {
        self.age += 1;
        self
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

    let mut p = Person::new("Taro", 20);
    p.say_name().say_age();
    p.take_age().say_name().say_age();
}
