use std::task::Poll;

use futures::{executor::block_on, future::join_all, Future};

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready("Zero!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn print_result(value: i32) {
    println!("{}", value);
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    print_result(ans1); // .awaitしない限り、中身の評価が走らないので、出力されない。
    let ans2 = async_add(3, 4).await;
    print_result(ans2).await;
    let ans3 = async_add(5, 4).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    async move {
        // move キーワードで、変数の所有権をasyncブロックの中に移する
        println!("{}", outside_variable);
    }
}

fn some_great_function() -> impl Future<Output = i32> {
    async {
        // asyncブロック内に置くことで、valueの所有権を'staticに引きのばす
        let value = 5;
        send_to_another_thread_with_borrowing(&value).await
        // borrowed value does not live long enough
    }
}

async fn send_to_another_thread_with_borrowing(x: &i32) -> i32 {
    *x
}

fn main() {
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }

    block_on(something_great_async_function()); // executor::block_on関数が async fnを実行するランタイムの起動ポイント

    block_on(move_to_async_block());
    let _ = some_great_function();
}
