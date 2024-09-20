use core::time;

mod test_channel;

mod test_multi_runtime_channel;

#[tokio::main]
async fn main() {
    console_subscriber::init();
    // tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    for _ in 0..5 {
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(50)).await;
        });
    }
    tokio::time::sleep(std::time::Duration::from_secs(100)).await;

    println!("Hello, world!");
}


