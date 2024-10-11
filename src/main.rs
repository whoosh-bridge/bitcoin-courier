mod indexer;
mod args;
mod masterkey;
mod enums;
mod orderbook;
mod gateway;
mod cache;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use crossbeam::{channel::{unbounded,Sender}, queue::SegQueue};
use indexer::{BitcoinListener,ReceivedPayment};
use enums::{NetworkType,Token};
use args::Args;
use masterkey::MasterKey;
use nakamoto::p2p::fsm::output::Connect;

fn main() -> Result<()> {
    if let Result::Ok(args) = Args::try_parse(){
        let bitcoin_network = match args.network{
            NetworkType::Mainnet => nakamoto::common::bitcoin::Network::Bitcoin,
            NetworkType::Testnet => nakamoto::common::bitcoin::Network::Testnet
        };

        let (receive_address_sender,receive_address_receiver) = unbounded(); 
        let (order_sender,order_receiver) = unbounded();
    
        let listener = BitcoinListener::new(bitcoin_network);
        let listener_handle  = listener.run(receive_address_receiver,order_sender)?;
        listener_handle.join();

        return Ok(())
    }
    
    Ok(())
}




