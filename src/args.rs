use clap::Parser;
use crate::enums::NetworkType;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"),
       about = "Serve a TODO list application.")]
pub struct Args {
    /// Address to bind the server to.
    #[clap(short, long /*, env = "SERVER_ADDRESS"*/, default_value = "0.0.0.0") ]
    pub address: String,

    /// Port to listen on.
    #[clap(short, long /*, env = "SERVER_PORT"*/, default_value = "3000") ]
    pub port: u16,    

    #[clap(value_enum, default_value = "testnet") ]
    pub network: NetworkType,

    #[clap(short, long /*, env = "SERVER_PORT"*/, default_value = "156.255.1.32:6379") ]
    pub redis: String,    

}