use std::time::Duration;
use trpl::Either;

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    let time = trpl::sleep(max_time);
    match trpl::select(future_to_try, time).await {
        Either::Left(r) => Ok(r),
        Either::Right(_) => Err(max_time),
    }
}
