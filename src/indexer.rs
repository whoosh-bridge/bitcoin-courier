type Reactor = nakamoto::net::poll::Reactor<net::TcpStream>;
use anyhow::{Ok, Result};
use nakamoto::chain::Transaction;
use nakamoto::client::traits::Handle;
use nakamoto::client::{Client, Config, Event, Network};
use nakamoto::common::bitcoin::blockdata::script::Builder;
use nakamoto::common::bitcoin::secp256k1::{Message, Secp256k1, SecretKey};
use nakamoto::common::bitcoin::{Address, OutPoint, PackedLockTime, Script, Sequence, TxIn, TxOut, Txid, Witness};
use nakamoto::common::bitcoin_hashes::hex::ToHex;
use nakamoto::common::network::Services;
use serde::ser::SerializeStruct;
use std::net;
use std::str::FromStr;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crossbeam::channel::{Receiver, Sender};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ReceiveAddress {
    pub script: Script,
    pub address: Address,
    pub account_index: u32,
    pub address_index: u32,
}

impl Serialize for ReceiveAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ReceiveAddress", 4)?;
        state.serialize_field("script", &self.script.to_string())?;
        state.serialize_field("address", &self.address.to_string())?;
        state.serialize_field("account_index", &self.account_index)?;
        state.serialize_field("address_index", &self.address_index)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ReceiveAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ReceiveAddressHelper {
            script: String,
            address: String,
            account_index: u32,
            address_index: u32,
        }

        // Deserialize the fields into a helper struct
        let helper = ReceiveAddressHelper::deserialize(deserializer)?;
        // Convert the strings to the appropriate types
        let script = Script::from_str(&helper.script).map_err(|_| {
            serde::de::Error::custom(format!("Invalid script format: {}", helper.script.as_str()))
        })?; // Handle error properly
        let address = Address::from_str(&helper.address).map_err(|_| {
            serde::de::Error::custom(format!(
                "Invalid address format: {}",
                helper.address.as_str()
            ))
        })?; // Handle error properly

        std::result::Result::Ok(ReceiveAddress {
            script,
            address,
            account_index: helper.account_index,
            address_index: helper.address_index,
        })
    }
}

pub struct ReceivedPayment {
    address: ReceiveAddress,
    amount: u64,
    block_height: u64,
}

pub struct BitcoinListener {
    // seed: [u8;32],
    // masterkey: MasterKey,
    // watch_list: Vec<Script>,
    // address_list: Vec<ReceiveAddress>,
    // account_index: u32,
    // address_index: u32,
    network: nakamoto::common::bitcoin::Network,
}

impl BitcoinListener {
    pub fn new(network: nakamoto::common::bitcoin::Network) -> BitcoinListener {
        BitcoinListener { network }
    }

    pub fn transfer(recipient_address: Address,amount: u64,signer: SecretKey){
        // Example input from a previous transaction (UTXO)
        
        let txid = Txid::from_str("your_input_txid_here").unwrap();
        let vout = 0;  // Index of the output in the UTXO

        // Input script and sequence
        let script_sig = Script::new();
        todo!("Add signature to the input");
        let sequence = Sequence::from_consensus(0xFFFFFFFF);

        let txin = TxIn {
            previous_output: OutPoint::new(txid, vout),
            script_sig,
            sequence,
            witness: Witness::new(),
        };

        // // Example output to a recipient address
        // let amount = 50_000;  // Amount in satoshis
        // let recipient_address = Address::from_str("your_bitcoin_address_here").unwrap();
        let script_pubkey = recipient_address.script_pubkey();

        let txout = TxOut {
            value: amount,
            script_pubkey,
        };

        // Create the transaction
        let tx = Transaction {
            version: 1,
            lock_time: PackedLockTime::ZERO,
            input: vec![txin],
            output: vec![txout],
        };

        // Signing the transaction
        let secp = Secp256k1::new();

        // Compute the signature hash for the input
        let sighash = tx.signature_hash(0, &script_pubkey, 0);  // Adjust as needed
        let msg = Message::from_slice(&sighash[..]).expect("32-byte hash");

        let signature = secp.sign(&msg, &signer);

        // Serialize the signature to DER format and append SIGHASH_ALL flag
        let mut sig_with_sighash = signature.serialize_der().to_vec();
        sig_with_sighash.push(0x01);  // Adding the SIGHASH_ALL flag

        // Create the scriptSig: [signature] [public key]
        let script_sig = Builder::new()
            .push_slice(&sig_with_sighash)
            .push_slice(&pubkey.serialize())
            .into_script();

        tx.input[0].script_sig = script_sig;
        client.handle().submit_transaction(tx);
    }

    pub fn run(
        &self,
        receive_addresses: Receiver<ReceiveAddress>,
        received_payments: Sender<ReceivedPayment>,
    ) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
        // Create a client using the above network reactor.
        let client = Client::<Reactor>::new()?;

        let client_handle = client.handle();
        let config_network = match self.network {
            nakamoto::common::bitcoin::Network::Testnet => Network::Testnet,
            nakamoto::common::bitcoin::Network::Bitcoin => Network::Mainnet,
            nakamoto::common::bitcoin::Network::Regtest => Network::Regtest,
            nakamoto::common::bitcoin::Network::Signet => Network::Signet,
        };
        // Run the client on a different thread, to not block the main thread.
        let config = Config::new(config_network);
        let peers_thread = thread::spawn(|| client.run(config).unwrap());
        // Wait for the client to be connected to a peer.

        

        let peers = client_handle.wait_for_peers(5, Services::default())?;

        for peer in peers {
            println!("{}:{}", peer.0, peer.1);
        }

        let handle = thread::spawn(move || -> anyhow::Result<()> {
            let client_handle_ref = client_handle.clone();
            let receive_addresses_ref = receive_addresses.clone();
            let mut watch_list = vec![];
            let mut addresses = vec![];

            let (tip, tip_header) = client_handle_ref.get_tip()?;

            client_handle_ref.rescan(3009441.., watch_list.iter().cloned());

            let events_queue = client_handle_ref.events();

            loop {
                if let Result::Ok(addr) = receive_addresses_ref.try_recv() {
                    watch_list.push(addr.script.clone());
                    addresses.push(addr);
                }
                if let Result::Ok(event) = events_queue.try_recv() {
                    match event {
                        Event::BlockConnected {
                            header,
                            hash,
                            height,
                        } => {
                            println!("block height:{height}");
                        }
                        Event::BlockMatched {
                            hash,
                            header,
                            height,
                            transactions,
                        } => {
                            let confirmations = tip - height + 1;
                            extract_user_payments(
                                &mut addresses,
                                transactions,
                                height,
                                received_payments.clone(),
                            );
                        }
                        _ => (),
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }

            Ok(())
        });

        Ok(handle)
    }
}

fn extract_user_payments(
    receive_addresses: &mut Vec<ReceiveAddress>,
    transactions: Vec<Transaction>,
    block_height: u64,
    received_payments: Sender<ReceivedPayment>,
) {
    for tx in transactions {
        let tx_hash = tx.txid().to_hex();
        for txo in tx.output {
            let script = txo.script_pubkey;
            for i in 0..receive_addresses.len() {
                let address_ptr = receive_addresses.pop().unwrap();
                if script.to_string() == address_ptr.address.script_pubkey().to_string() {
                    received_payments.send(ReceivedPayment {
                        address: address_ptr,
                        amount: txo.value,
                        block_height,
                    });
                    // println!("Your fund for address {receive_address} received: hash -> {tx_hash} confirmations: {confirmations}");
                }
            }
        }
    }
}
