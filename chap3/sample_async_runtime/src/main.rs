use async_trait::async_trait;

async fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[async_trait]
trait AsyncTrait {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait for Runner {
    async fn f(&self) {
        println!("Hello, async-trait");
    }
}

#[async_std::main]
async fn main() {
    let ans = add(2, 3).await;
    println!("{}", ans);

    let runner = Runner {};
    runner.f().await;
}
