use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1", env = "HOST")]
    pub host: String,

    #[arg(long, default_value_t = 11211, env = "PORT")]
    pub port: u16,
}
