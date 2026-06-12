use mini_redis::client;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        // .filter(|msg| match msg {
        //     Ok(msg) if msg.content.len() == 1 => true,
        //     _ => false,
        // })
        // .map(|msg| msg.unwrap().content)
        .filter_map(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => Some(msg.content),
            _ => None,
        })
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE");

    Ok(())
}

// #####################################################

// use tokio_stream::Stream;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// use std::time::Duration;

// struct Interval {
//     rem: usize,
//     delay: Delay,
// }

// impl Interval {
//     fn new() -> Self {
//         Self {
//             rem: 3,
//             delay: Delay { when: Instant::now() }
//         }
//     }
// }

// impl Stream for Interval {
//     type Item = ();

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
//         -> Poll<Option<()>>
//     {
//         if self.rem == 0 {
//             // No more delays
//             return Poll::Ready(None);
//         }

//         match Pin::new(&mut self.delay).poll(cx) {
//             Poll::Ready(_) => {
//                 let when = self.delay.when + Duration::from_millis(10);
//                 self.delay = Delay { when };
//                 self.rem -= 1;
//                 Poll::Ready(Some(()))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

// #####################################################

// use async_stream::stream;
// use std::time::{Duration, Instant};

// stream! {
//     let mut when = Instant::now();
//     for _ in 0..3 {
//         let delay = Delay { when };
//         delay.await;
//         yield ();
//         when += Duration::from_millis(10);
//     }
// }