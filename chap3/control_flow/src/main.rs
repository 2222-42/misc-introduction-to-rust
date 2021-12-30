use std::{io::Write, vec};

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

#[derive(Debug)]
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

enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion,
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    fn get_happy(&mut self) -> String {
        self.state = Emotion::Happy;
        format!("{} is always happy.", self.name)
    }

    fn tell_state(&self) -> String {
        todo!()
    }
}

fn sample_unreachable(x: usize) -> &'static str {
    match x {
        n if n * n % 3 == 0 => "3n",
        n if n * n % 3 == 1 => "3n+1 or 3n+2",
        _ => unreachable!(), // コンパイラは上記条件で網羅していることを判定できないため
    }
}

// EqにはPartialEqが必要(PartialEqは、a == aは満たさなくても良いと条件を緩めたEq)
#[derive(PartialEq, Eq)]
struct A(i32);

// PartialOrdのためにはPartialEqが必要
#[derive(PartialEq, PartialOrd)]
struct B(f32); // f32は厳密なEqが実装できない。

// Copyには Cloneが必要
#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Default)]
struct F;

fn sort_f32_in_vec() {
    let mut x = vec![0.1, 0.5, 0.3, 0.4, 0.2]; // vec![0.1, 0.5, 0.3, 0.4, 0.2, 0.0 / 0.0];
    x.sort_by(|a, b| a.partial_cmp(b).unwrap()); // NaNが入っているとunwrapでpanicが起きる
    println!("{:?}", x);
}

fn main() {
    sort_f32_in_vec();
    println!("{:?}", A(0) == A(1)); // 一致比較可能
    println!("{:?}", B(1.0) > B(0.0)); // 大小比較可能
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // moveではなくcopy
    let d0 = D;
    let _d1 = d0.clone();
    let _f = F::default();

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
    println!("{:?}", p);

    println!("hello {}", p.name);
    eprintln!("hello {}", p.name); // 標準エラー出力

    let mut w = Vec::new();
    let _ = write!(&mut w, "{}", "ABC");
    let _ = writeln!(&mut w, " is 123");
    dbg!(w); // file name, line number, formula, value of formula
    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CAARGO_HOME: {}", env!("CARGO_HOME"));

    // debug_assert!(false); // --release リリースビルドの場合はスキップされる

    let mut happy_person = HappyPerson {
        name: "Takeshi".to_string(),
        state: Emotion::Happy,
    };

    println!("{}", happy_person.get_happy());
    // println!("{}", happy_person.get_anger()); // thread 'main' panicked at 'not implemented', src/main.rs:70:9

    println!("{}", sample_unreachable(3));
    println!("{}", sample_unreachable(4));
    println!("{}", sample_unreachable(5));
}
