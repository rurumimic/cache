use tokio::signal;
use tokio::time;
use tokio::task::JoinHandle;
use tokio::sync::{mpsc, broadcast, oneshot};
use tokio_util::sync::CancellationToken;

async fn task(token: CancellationToken, i: u64) -> u64 {
    println!("Task {}.", i);

    tokio::select! {
        _ = token.cancelled() => {
            println!("Task {} shutting down.", i);
        },
        _ = time::sleep(time::Duration::from_secs(3)) => {
            println!("Timeout 3 sec");
        }
    }

    i
}

#[tokio::main]
async fn main() {
    println!("Hello!");

    let mut handles: Vec<JoinHandle<u64>> = Vec::new();

    let cancel_token = CancellationToken::new();

    for i in 1..10 {
        let token = cancel_token.clone();

        let handle = tokio::spawn(async move {
          task(token, i).await
        });

        handles.push(handle);
    }

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Shutdown! CTRL-C");
            cancel_token.cancel();
        },
        _ = time::sleep(time::Duration::from_secs(2)) => {
            println!("Timeout!");
            cancel_token.cancel();
        },
    }

    for handle in handles {
        let ret = handle.await;
        match ret {
            Ok(ok) => println!("Ok: {:?}", ok),
            Err(error) => println!("Err: {:?}", error),
        }
    }

    println!("Bye.");
}

