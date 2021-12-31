use std::{
    sync::{Arc, Mutex},
    thread,
};

fn pass_closure() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10])); // スレッド間で所有権を共有するためにRcを使う
                                                  // -> マルチスレッドでの同時アクセスを可能にするためにRcではなく、Arcを使う
                                                  // -> Arcでは書き換えを行えないので、別途、排他制御を行うためにMutexを使う
    for x in 0..10 {
        let data_ref = data.clone(); // Rcの参照カウンタを増やす。
                                     // →　Arcの参照カウントを増やす。Rcよりコストが余分にかかる。

        // クロージャがデフォルトでは参照をキャプチャするので、moveキーワードを使ってxの所有権をスレッドに移す
        handles.push(thread::spawn(move || {
            // lockを使いdataへの可変参照を得る
            // lockは複数のスレッドが同時にアクセスしないことを保証するためのメカニズム。あるスレッドが取得したら他のスレッドは取得できない。
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
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
