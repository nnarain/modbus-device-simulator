//
// cli.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version ,about, long_about = None)]
pub struct Args {
    /// IP address to use
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,
    /// Port to use
    #[arg(short, long, default_value_t = 5502)]
    pub port: u16,
    /// Device script
    #[arg(short, long)]
    pub script: String
}
