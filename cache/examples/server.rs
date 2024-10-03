use clap::Parser;
use tokio::signal;
use tokio::time;
use tokio_util::sync::CancellationToken;

use tracing::info;

use cache::config;
use cache::server;

#[tokio::main]
async fn main() -> cache::Result<()> {
    let args = config::Args::parse();

    /* init */
    cache::logging::trace_init()?;
    let listener = cache::network::network_init(&args.host, args.port).await?;

    println!("telnet {} {}", args.host, args.port);

    /* event loop */
    let cancel = CancellationToken::new();
    let handle = tokio::spawn({
        let cancel = cancel.clone();
        async move {
            server::run(cancel, listener).await;
        }
    });

    /* handle server */
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Shutdown! CTRL-C");
            cancel.cancel();
        },
        _ = time::sleep(time::Duration::from_secs(300)) => {
            cancel.cancel();
        },
    }

    let _ = handle.await;

    println!("Bye.");

    Ok(())
}
