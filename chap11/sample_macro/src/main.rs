use tomlstruct::tomlstruct;

// Rustのマクロ置換は構文解析後の構文木に対しておこな合われるので、展開前に評価された演算子の優先順位が展開後も維持される。
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! vec_one_item {
    ($x:expr) => {{
        let mut temp_vec = Vec::new();
        temp_vec.push($x);
        temp_vec
    }};
}

macro_rules! vec_multi_item {
    ($x:ty) => {{
        let temp_vec: Vec<$x> = Vec::new();
        temp_vec
    }};
    ($($x:expr), *) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($x);)*
        temp_vec
    }};
}

tomlstruct! {
    [Hello]
    name = "hello"
    version = 1.0
}

fn main() {
    assert_eq!(25, five_times!(2 + 3));
    println!("vec_one_item!: {:?}", vec_one_item![1]);
    println!("vec_multi_item!: {:?}", vec_multi_item![1, 2, 3]);
    println!("vec_multi_item!: {:?}", vec_multi_item![i32]);

    let _ = Hello {
        name: String::from("hello"),
        version: 1.0,
    };
}
