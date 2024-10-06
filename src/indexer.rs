
type Reactor = nakamoto::net::poll::Reactor<net::TcpStream>;
use nakamoto::chain::Transaction;
use nakamoto::client::traits::Handle;
use nakamoto::client::{Client, Config, Event, Network};
use nakamoto::common::bitcoin::bech32::ToBase32;
use nakamoto::common::bitcoin::secp256k1::ffi::Context;
use nakamoto::common::bitcoin::secp256k1::{Secp256k1, SecretKey};
use nakamoto::common::bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use nakamoto::common::bitcoin::{Address, KeyPair, PrivateKey, PublicKey};
use nakamoto::common::bitcoin_hashes::hex::ToHex;
use nakamoto::common::{bitcoin::network::constants::ServiceFlags,network::Services};
use std::borrow::Borrow;
use std::hash::Hash;
use std::io::Write;
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;
use std::thread;
use std::net;
use std::collections::HashSet;
use anyhow::Result;

use crate::keygen::MasterKey;


pub fn run_indexer () -> anyhow::Result<()>{
    // Create a client using the above network reactor.
    let client = Client::<Reactor>::new()?;
    let network = nakamoto::common::bitcoin::Network::Testnet;
    
    let client_handle = client.handle();

    
    // Run the client on a different thread, to not block the main thread.
    let config = Config::new(Network::Testnet);    
    thread::spawn(|| client.run(config).unwrap());    
    // Wait for the client to be connected to a peer.
    
    let peers = client_handle.wait_for_peers(5, Services::default())?;    

    for peer in peers{
        println!("{}:{}",peer.0,peer.1);                
    }    

    let seed = b"Qu/E,.qp40ruLCX8GDSYlE2m?:I[|}5,";
    let masterkey = MasterKey::new(*seed,network);
    let addresses: &mut HashSet<Address> = &mut HashSet::new();

    let wallet_index:u32 = 0;
    let last_index = (2u64.pow(32) - 1u64) as u32;    

    for address_index in 0u32..5{
        let receive_address = masterkey.new_bitcoin_receive_address(0, address_index);
        addresses.insert(receive_address.clone());    
        println!("Receive address {} added to watch list",receive_address);
    } 
    return Ok(());
    

    let watch_list: Vec<_> = addresses.iter().map(|a| a.script_pubkey()).collect();    
    client_handle.rescan(3009441..,watch_list.iter().cloned());    
    
    
    let events_queue = client_handle.events();            
    
    loop{                
        let event = events_queue.recv()?;                


        match event {
            Event::PeerConnected { addr, link } => {
                client_handle.get_filters(RangeInclusive::new(3,5));
            },
            Event::BlockConnected { header, hash, height } => {
                println!("block height:{height}");
            },            
            Event::PeerDisconnected { addr, reason } => println!("Peer {addr} disconnected."),
            Event::PeerHeightUpdated { height } => println!("Block {height} updated!"),
            Event::Synced { height, tip }=> {

            },
            Event::TxStatusChanged { txid, status } => println!("Transaction {txid} updated to {status} "),            
            Event::Ready { tip, filter_tip }=>println!("Ready event!"),
            Event::PeerConnectionFailed { addr, error }=>println!("Peer connection failed!"),
            Event::BlockDisconnected { header, hash, height } => println!("Block disconnected!"),
            Event::PeerNegotiated { addr, link, services, height, user_agent, version } => println!("Peer negotiated!"),
            Event::BlockMatched { hash, header, height, transactions } =>{
                println!("Block matched {height}");
                
                for tx in transactions{
                    let tx_hash =  tx.txid().to_hex();
                    println!("transaction hash: {tx_hash}");
                    
                    for txi in tx.input {
                        
                        match Address::from_script(&txi.script_sig,network){
                            Ok(addr) =>println!("Transaction source {}",addr.to_string()),
                            _=>()
                        }
                        
                    }
                    for txo in tx.output{
                        match Address::from_script(&txo.script_pubkey,network){
                            Ok(addr)=>println!("Transaction received {} satoshi from {}",txo.value,addr.to_string()),
                            _=>()
                        }
                        
                    }
                    
                }
            }
            Event::FilterProcessed { block, height, matched, valid } => {
                print!(".");
                std::io::stdout().flush();
            },    
            _=>println!("Other events")
        }        
    }


    Ok(())
}