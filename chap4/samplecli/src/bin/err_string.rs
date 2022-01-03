use std::fs::read_to_string;

fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";
    let num_str = read_to_string(path).map_err(|err| err.to_string())?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2) // `parse()`の結果が`Ok(t)`の場合のみ実行し、`Ok(t*2)`となる
        .map_err(|e| e.to_string()) // `parse()`の結果が`Err(e)`の場合のみ`eの文字列表現を返す`
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
