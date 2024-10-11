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

pub struct MasterKey{
  seed: [u8; 32],
  master_key: ExtendedPrivKey,
  secp: Secp256k1<nakamoto::common::bitcoin::secp256k1::All>,
  network: nakamoto::common::bitcoin::Network
}

impl MasterKey {
  pub fn new(seed: [u8; 32],network: nakamoto::common::bitcoin::Network)-> MasterKey{
      let secp = Secp256k1::new();

      MasterKey{
          seed,
          secp,
          network,
          master_key: ExtendedPrivKey::new_master(network, &seed).unwrap()
      }        
  }

  pub fn new_bitcoin_receive_address(&self,account_index: u32, address_index: u32) -> Address{
    let change: u32 = 0;
    let account_index = 0;
    let coin_type= 0;
    let path = format!("m/44'/{coin_type}'/{account_index}'/{change}/{address_index}");
    let derivation_path = DerivationPath::from_str(&path).unwrap();
    // Derive the child private key
    let child_key = self.master_key.derive_priv(&self.secp, &derivation_path).unwrap();
    let private_key = child_key.private_key;
    let public_key = PublicKey::from_str(private_key.public_key(&self.secp).to_string().as_str()).unwrap() ;        
    //   let receive_address = Address::p2pkh(&public_key, self.network);
    let receive_address = Address::p2wpkh(&public_key, self.network).unwrap();
    receive_address
  }

  pub fn new_bitcoin_receive_qrcode(&self,bitcoin_address: Address,amount: u32, label: &str, message: &str)->String{      
      let uri = format!(
          "bitcoin:{}?amount={}&label={}&message={}",
          bitcoin_address, amount, label, message
      );
      uri
  }
}