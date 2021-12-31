use std::thread;

fn pass_closure() {
    let mut handles = Vec::new();
    for x in 0..10 {
        // クロージャがデフォルトでは参照をキャプチャするので、moveキーワードを使ってxの所有権をスレッドに移す
        handles.push(thread::spawn(move || {
            println!("Hellow, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn main() {
    // スレッドのハンドル
    let handle = thread::spawn(|| // クロージャを受け取り、それを新しいスレッドで実行する
        {
        println!("Hello, world!");
    });

    let _wait = dbg!(handle.join()); // joinを読んで、スレッドの終了を待つ。

    pass_closure();
}
