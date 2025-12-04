use std::time::Duration;

fn main() {
    trpl::run(async {
        let fut1 = async {
            println!("Start first task!");
            for i in 1..100 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(0)).await;
            }
            println!("End first task!");
        };

        let fut2 = async {
            println!("Start second task!");
            for i in 1..100 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(0)).await;
            }
            println!("End second task!");
        };
        trpl::sleep(Duration::from_millis(2000)).await;
        println!("Hello, world!");
        trpl::join(fut1, fut2).await;
    });
}
