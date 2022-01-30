use std::panic::{catch_unwind, resume_unwind};

fn print_range(x: &[i32]) {
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

struct A(bool);
impl Drop for A {
    // Drop処理内ではパニックが起きそうな処理を避けること(回復処理が行われなくなる)
    fn drop(&mut self) {
        if self.0 {
            eprintln!("A::drop");
        }
    }
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

    let mut flag = A(true);
    assert!(false);
    flag.0 = false;
    eprintln!("Success!");

    let result = catch_unwind(|| {
        assert!(false);
    });
    if let Err(payload) = result {
        eprintln!("assert!(false) failed");
        // パニック処理を続行する　→ eprintln!("success");は実行されない
        resume_unwind(payload); // resume_unwindが役立つケースのひとつはFFI教会を超える必要がる場合。
    } else {
        eprintln!("assert!(false) succeeded");
    }

    eprintln!("success");
}
