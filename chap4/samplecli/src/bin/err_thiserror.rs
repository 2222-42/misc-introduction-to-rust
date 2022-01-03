use std::fs::read_to_string;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("failed to read string from {0}")] // fmt::Displayトレイトを実装するのと同様
    ReadError(String),
    #[error(transparent)] // 発生元のエラーをそのまま使う
    ParseError(#[from] std::num::ParseIntError), // #[from]アトリビュートで自動的にFrom<T>トレイとも実装される
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    let num_str = read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2) // `parse()`の結果が`Ok(t)`の場合のみ実行し、`Ok(t*2)`となる
        .map_err(MyError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
