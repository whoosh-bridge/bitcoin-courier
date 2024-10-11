
type Reactor = nakamoto::net::poll::Reactor<net::TcpStream>;
use crossbeam::queue::{ArrayQueue, SegQueue};
use nakamoto::chain::Transaction;
use nakamoto::client::traits::Handle;
use nakamoto::client::{Client, Config, Event, Network};
use nakamoto::common::bitcoin::bech32::ToBase32;
use nakamoto::common::bitcoin::secp256k1::ffi::Context;
use nakamoto::common::bitcoin::secp256k1::{Secp256k1, SecretKey};
use nakamoto::common::bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use nakamoto::common::bitcoin::{Address, KeyPair, PrivateKey, PublicKey, Script};
use nakamoto::common::bitcoin_hashes::hex::ToHex;
use nakamoto::common::{bitcoin::network::constants::ServiceFlags,network::Services};
use std::borrow::Borrow;
use std::hash::Hash;
use std::io::Write;
use std::iter::Map;
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;
use std::thread::{self, JoinHandle};
use std::{clone, net};
use std::collections::HashSet;
use anyhow::{Ok, Result};
use std::time::Duration;

use crate::masterkey::MasterKey;

use std::sync::Arc;
use crossbeam::channel::{unbounded, Receiver, Sender};

#[derive(Clone)]
pub struct ReceiveAddress{
  script: Script,
  address: Address
}


#[derive(Clone)]
pub struct ReceivedPayment{
  address: ReceiveAddress,
  amount: u64,
  block_height: u64
}


pub struct BitcoinListener{
  // seed: [u8;32],
  // masterkey: MasterKey,
  // watch_list: Vec<Script>,
  // address_list: Vec<ReceiveAddress>,
  // account_index: u32,
  // address_index: u32,
  network: nakamoto::common::bitcoin::Network
}


impl BitcoinListener{
  pub fn new(network : nakamoto::common::bitcoin::Network) -> BitcoinListener{
    
    
    BitcoinListener{
      network               
    }
  }

  pub fn run (&self, receive_addresses: Receiver<ReceiveAddress>,received_payments: Sender<ReceivedPayment> ) -> anyhow::Result<JoinHandle<anyhow::Result<()>>>{
    
    // Create a client using the above network reactor.
    let client = Client::<Reactor>::new()?;
    
    let client_handle = client.handle();
    let config_network = match self.network {
      nakamoto::common::bitcoin::Network::Testnet => Network::Testnet,
      nakamoto::common::bitcoin::Network::Bitcoin => Network::Mainnet,
      nakamoto::common::bitcoin::Network::Regtest => Network::Regtest,
      nakamoto::common::bitcoin::Network::Signet => Network::Signet
    };
    // Run the client on a different thread, to not block the main thread.
    let config = Config::new(config_network);    
    let peers_thread = thread::spawn(|| client.run(config).unwrap());    
    // Wait for the client to be connected to a peer.
    
    let peers = client_handle.wait_for_peers(5, Services::default())?;    

    for peer in peers{
        println!("{}:{}",peer.0,peer.1);                
    } 

    let handle = thread::spawn(move || -> anyhow::Result<()>{
      
      let client_handle_ref = client_handle.clone(); 
      let receive_addresses_ref = receive_addresses.clone();
      let mut watch_list = vec![];
      let mut addresses = vec![];   

      let (tip,tip_header) = client_handle_ref.get_tip()?;

      client_handle_ref.rescan(3009441..,watch_list.iter().cloned());    
                
      let events_queue = client_handle_ref.events();       

      loop{                
        if let Result::Ok(addr) = receive_addresses_ref.try_recv(){
          addresses.push(addr);
        }
        if let Result::Ok(event) = events_queue.try_recv(){
          match event {
            Event::BlockConnected { header, hash, height } => {
              println!("block height:{height}");
            },            
            Event::BlockMatched { hash, header, height, transactions } =>{
              let confirmations = tip - height + 1;
              let payments = extract_user_payments(&mut addresses, transactions,height,received_payments.clone());     
            }
            _=>()        
          } 
        }       
        thread::sleep(Duration::from_millis(100));        
      }

      Ok(())
    });

    Ok(handle)
  }
}

fn extract_user_payments(receive_addresses:&mut Vec<ReceiveAddress>,transactions: Vec<Transaction>,block_height: u64,received_payments: Sender<ReceivedPayment>) {
  for tx in transactions{
    let tx_hash =  tx.txid().to_hex();
    for txo in tx.output{    
      let script = txo.script_pubkey;                                        
      for i in 0..receive_addresses.len(){
        let address_ptr=  receive_addresses.pop().unwrap(); 
        if script.to_string() == address_ptr.address.script_pubkey().to_string() {
          received_payments.send(ReceivedPayment{
            address: address_ptr,
            amount: txo.value,
            block_height: block_height
          });          
          // println!("Your fund for address {receive_address} received: hash -> {tx_hash} confirmations: {confirmations}");
        }                        
      }                        
    }                                   
  }
}

