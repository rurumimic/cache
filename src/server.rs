use tokio::io::AsyncReadExt;
use tokio::io::BufWriter;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio_util::bytes::BytesMut;
use tokio_util::sync::CancellationToken;

use tracing::{info, error};

use std::io;

pub async fn run(cancel: CancellationToken, listener: TcpListener) {
    loop {
        tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok((s, a)) => handler(cancel.clone(), s, a).await,
                    Err(_) => break,
                };
            },
            _ = cancel.cancelled() => {
                break;
            }
        }

    }
}

async fn handler(cancel: CancellationToken, socket: TcpStream, _addr: SocketAddr) {
    tokio::spawn(async move {
        task(cancel, socket).await
    });
}

#[tracing::instrument]
async fn task(cancel: CancellationToken, socket: TcpStream) {
    let mut buf = BytesMut::with_capacity(1024);
    let mut socket = BufWriter::new(socket);

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                return;
            },
            nread = socket.read_buf(&mut buf) => {
                match nread {
                    Ok(0) => { 
                        break;
                    },
                    Ok(_n) => println!("{:?}", buf),
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    },
                    Err(e) => {
                        error!("Error: {:?}", e);
                        break;
                    },
                }

                buf.clear();
            },
        }
    }
}

