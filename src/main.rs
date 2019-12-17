use async_std::sync::channel;
use std::error::Error;
use tokio::time::{delay_for, interval};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel(1);
    tokio::spawn(async move {
        println!("received: {:?}", rx.recv().await);
        let mut interval = interval(Duration::from_secs(1));
        loop { 
            interval.tick().await;
            println!("hello");
        };
    });
    tx.send(1u32).await;
    delay_for(Duration::from_secs(10)).await;
    Ok(())
}
