use std::pin::{pin, Pin};
use std::time::Duration;
use trpl;

fn main() {
    trpl::block_on(async {
        let tx_fut = pin!(async move {
            trpl::sleep(Duration::from_millis(2000)).await;
        });

        let tx1_fut = pin!(async move {
            // --snip--
        });

        let rx_fut = pin!(async move {
            trpl::sleep(Duration::from_millis(2000)).await;
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        let bbb = Box::new(Box::new(Box::new(33)));
        let vvv = Pin::new(vec!["dsd", "sdsdsd"]);

        let ffff = vvv;

        titi(&bbb);

        trpl::join_all(futures).await;
    });
}

fn titi(i: &i32) {}
