use clap::{Parser,ValueEnum};


#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"),
       about = "Serve a TODO list application.")]
struct Args {
    /// Address to bind the server to.
    #[clap(short, long /*, env = "SERVER_ADDRESS"*/, default_value = "0.0.0.0") ]
    address: String,

    /// Port to listen on.
    #[clap(short, long /*, env = "SERVER_PORT"*/, default_value = "3000") ]
    port: u16,

    #[clap(value_enum, default_value = "testnet") ]
    network: NetworkTpe
}