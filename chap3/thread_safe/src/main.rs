use std::thread;

fn main() {
    // スレッドのハンドル
    let handle = thread::spawn(|| // クロージャを受け取り、それを新しいスレッドで実行する
        {
        println!("Hello, world!");
    });

    let _wait = dbg!(handle.join()); // joinを読んで、スレッドの終了を待つ。
}
