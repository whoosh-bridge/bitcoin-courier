mod indexer;
mod args;
mod masterkey;
mod enums;
mod orderbook;
mod gateway;
mod cache;

use std::sync::{Arc, Mutex};

use anyhow::{Ok, Result};
use clap::Parser;
use crossbeam::channel::unbounded;
use indexer::BitcoinListener;
use enums::NetworkType;
use args::Args;
use crate::cache::Cache;
use gateway::run_gateway;

fn main() -> Result<()> {
    if let Result::Ok(args) = Args::try_parse(){
        let bitcoin_network = match args.network{
            NetworkType::Mainnet => nakamoto::common::bitcoin::Network::Bitcoin,
            NetworkType::Testnet => nakamoto::common::bitcoin::Network::Testnet
        };
        
        let cache = Arc::new(Mutex::new(Cache::initialize(args.network,args.redis)?));
        

        let (receive_address_sender,receive_address_receiver) = unbounded(); 
        let (order_sender,order_receiver) = unbounded();
    
        let listener = BitcoinListener::new(bitcoin_network);
        let listener_handle  = listener.run(receive_address_receiver,order_sender)?;

        run_gateway(cache,receive_address_sender);

        listener_handle.join();

        return Ok(())
    }
    
    Ok(())
}




