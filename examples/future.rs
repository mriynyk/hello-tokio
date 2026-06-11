use std::{
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
    thread::sleep,
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("poll");
        if Instant::now() >= self.when {
            println!("Done!");
            Poll::Ready("Done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[allow(unused)]
fn future_executer_example() {
    let when = Instant::now() + Duration::from_millis(10);

    let future = Delay { when };

    let mut cx = Context::from_waker(Waker::noop());
    let mut future_pinned = pin!(future);

    let mut count = 0;

    loop {
        match future_pinned.as_mut().poll(&mut cx) {
            Poll::Ready(_) => {
                println!("Ready!!");
                break;
            }
            Poll::Pending => {
                println!("{count}");
                count += 1;
                sleep(Duration::from_millis(1));
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // future_executer_example();

    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "Done");
}

#[allow(dead_code)]
enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => match Pin::new(my_future).poll(cx) {
                    Poll::Ready(out) => {
                        assert_eq!(out, "done");
                        *self = Terminated;
                        return Poll::Ready(());
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}
