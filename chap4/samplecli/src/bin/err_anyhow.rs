use anyhow::{bail, ensure, Context, Result};
use std::fs::read_to_string;

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";
    let num_str =
        read_to_string(path).with_context(|| format!("failed to rewad string from {}", path))?; // 引数はFnOnce型なのでクロージャを受け取れる。エラーが発生しなかったらその文字列を生成する処理を実行しない。

    if num_str.len() >= 10 {
        bail!("it may be too large number"); // 早期リターンする
    }

    ensure!(num_str.starts_with("1"), "first digit is not 1"); // 第一引数の評価結果がfalseならエラーを返すから、if式不要。

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2) // `parse()`の結果が`Ok(t)`の場合のみ実行し、`Ok(t*2)`となる
        .context("failed to parse string")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{:#?}", x),
        Err(e) => println!("{:#?}", e),
    }
}
