use tokio::signal;
use tokio::time;
use tokio_util::sync::CancellationToken;

use tracing::info;

use cache::server;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> cache::Result<()> {
    /* init */
    cache::trace_init()?;
    let listener = cache::network_init(cache::DEFAULT_HOST, cache::DEFAULT_PORT).await?;

    println!("telnet {} {}", cache::DEFAULT_HOST, cache::DEFAULT_PORT);

    /* event loop */
    let cancel = CancellationToken::new();
    let cancel2 = cancel.clone();
    let handle = tokio::spawn(async move {
        server::run(cancel2, listener).await;
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
