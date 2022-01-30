use std::panic::catch_unwind;

fn print_range(x: &[i32]) {
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

fn main() {
    let requests = vec![
        vec![1, 2, 3],
        vec![],
        vec![2000000000, -2000000000],
        vec![0, 42],
    ];

    for request in &requests {
        // catch_wind内での処理が失敗しても、次の処理へと移る
        let result = catch_unwind(|| {
            print_range(request);
        });

        if let Err(_payload) = result {
            eprintln!("print_range failed");
        } else {
            eprintln!("print_range succeeded");
        }
    }

    // パニックの有無はJoinHandler::joinのresultに入っているので、スレッド辞退がcatch_unwindと同等の機能を持っている
    let thread1 = std::thread::spawn(move || {
        for request in &requests {
            print_range(request);
        }
    });
    eprintln!("is_ok = {}", thread1.join().is_ok());

    let thread2 = std::thread::spawn(|| {
        assert!(true);
    });
    eprintln!("is_ok = {}", thread2.join().is_ok());
}
