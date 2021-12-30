trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooooohhh!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    let _t1 = make_tuple(1, 2);
    let _t2 = make_tuple("Hello", "World");
    let _t3 = make_tuple(vec![1, 2, 3], vec![4.5]);
    let _t4 = make_tuple(3, "years old");
}
