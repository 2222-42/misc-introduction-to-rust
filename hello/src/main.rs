use std::result::Result;

fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

type TestResult = Result<i32, String>;

fn error_handling(result: TestResult) -> TestResult {
    let code = result?;
    println!("code: {}", code);
    Ok(100)
}

fn main() {
    println!("Hello, world!");

    let result: Result<i32, String> = Ok(300);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    let result: TestResult = Ok(300);
    if let Ok(code) = result {
        println!("code: {}", code);
    }

    println!("code: {}", result.unwrap_or(-1));

    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    let result: TestResult = Ok(200);
    let next_result = result.and_then(func);
    let _last_result = error_handling(next_result);
    let result: TestResult = Err("error".to_string());
    let next_result = result.and_then(func);
    let _last_result = error_handling(next_result);
}
