//
// This program is a script-able a modbus device server
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 26 2022
//

use modbus_device_simulator::{
    cli::Args,
    server,
    device::Device,
};
use clap::Parser;
use anyhow::{Result, Context};

use std::{
    fs,
    net::SocketAddr
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    let sock_addr: SocketAddr = format!("{}:{}", args.ip, args.port).parse()
        .with_context(|| format!("Invalid host/port"))?;

    println!("Starting Modbus server on: {}", sock_addr);

    let script = fs::read_to_string(args.script)?;

    // Create the virtual device
    let device = Device::new(&script)?;

    // Start the server task
    tokio::spawn(server::run(sock_addr, device));
    // Wait for user exit
    tokio::signal::ctrl_c().await?;

    Ok(())
}
