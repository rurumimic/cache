use tokio::signal;
use tokio::time;
use tokio::task::JoinHandle;
// use tokio::sync::{mpsc, broadcast, oneshot};
use tokio_util::sync::CancellationToken;

use tracing::{info, error};

#[tokio::main]
async fn main() -> cache::Result<()> {
    println!("Hello!");

    cache::init_tracing()?;

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
        _ = time::sleep(time::Duration::from_secs(10)) => {
            cancel_token.cancel();
        },
    }

    for handle in handles {
        let ret = handle.await;
        match ret {
            Ok(ok) => info!("Ok: {:?}", ok),
            Err(error) => error!("Err: {:?}", error),
        }
    }

    println!("Bye.");

    Ok(())
}

#[tracing::instrument]
async fn task(token: CancellationToken, i: u64) -> u64 {
    println!("Task {}.", i);

    tokio::select! {
        _ = token.cancelled() => {
            info!("Task {} shutting down.", i);
        },
        _ = time::sleep(time::Duration::from_secs(30)) => {
            info!("Timeout 3 seconds.");
        }
    }

    i
}

