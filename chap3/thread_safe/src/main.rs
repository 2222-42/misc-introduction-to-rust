use std::{
    sync::{mpsc, Arc, Mutex},
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

fn message_passing() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // main -> threads
        let (snd_tx, snd_rx) = mpsc::channel();
        // thread -> main
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }))
    }

    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
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

    message_passing();
}
