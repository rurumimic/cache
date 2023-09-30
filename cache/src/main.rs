use tokio::io::AsyncReadExt;
use tokio::io::BufWriter;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::time;
use tokio::task::JoinHandle;
use tokio_util::bytes::BytesMut;
use tokio_util::sync::CancellationToken;

use tracing::{info, error};

use std::io;

async fn server(listener: TcpListener, cancel_token: CancellationToken) {
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();

    for i in 1..3 {
        tokio::select! {
            res = listener.accept() => {
                let (socket, _addr) = match res {
                    Ok((s, a)) => (s, a),
                    Err(_) => break,
                };

                let token = cancel_token.clone();

                let handle = tokio::spawn(async move {
                    task(token, i, socket).await
                });

                handles.push(handle);
            },
            _ = cancel_token.cancelled() => {
                break;
            }
        }

    }

    for handle in handles {
        let x = handle.await;
        info!("{:?}", x);
    }

}

#[tokio::main]
async fn main() -> cache::Result<()> {
    println!("Hello!");

    cache::init_tracing()?;

    let cancel_token = CancellationToken::new();
    let token = cancel_token.clone();

    let listener = cache::init_network("127.0.0.1", cache::DEFAULT_PORT).await?;

    let handle = tokio::spawn(async move {
        server(listener, token).await;
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Shutdown! CTRL-C");
            cancel_token.cancel();
        },
        _ = time::sleep(time::Duration::from_secs(10)) => {
            cancel_token.cancel();
        },
    }

    let _ = handle.await;
    println!("All connections are complete.");

    println!("Bye.");

    Ok(())
}

#[tracing::instrument]
async fn task(token: CancellationToken, i: u64, socket: TcpStream) -> u64 {
    let mut buf = BytesMut::with_capacity(1024);
    let mut socket = BufWriter::new(socket);

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                info!("Task {} shutting down.", i);
                return i;
            },
            nread = socket.read_buf(&mut buf) => {
                match nread {
                    Ok(0) => { 
                        info!("Client {} out.", i);
                        break;
                    },
                    Ok(n) => println!("{}: {:?} - {:?}", i, n, buf),
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    },
                    Err(e) => {
                        error!("{}: Error: {:?}", i, e);
                        break;
                    },
                }
            },
        }
    }

    i
}

