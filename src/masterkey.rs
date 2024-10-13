type Reactor = nakamoto::net::poll::Reactor<net::TcpStream>;
use nakamoto::common::bitcoin::secp256k1::Secp256k1;
use nakamoto::common::bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use nakamoto::common::bitcoin::{Address, PublicKey};
use std::net;
use std::str::FromStr;

pub struct MasterKey {
    seed: [u8; 32],
    master_key: ExtendedPrivKey,
    secp: Secp256k1<nakamoto::common::bitcoin::secp256k1::All>,
    network: nakamoto::common::bitcoin::Network,
}

impl MasterKey {
    pub fn new(seed: [u8; 32], network: nakamoto::common::bitcoin::Network) -> MasterKey {
        let secp = Secp256k1::new();

        MasterKey {
            seed,
            secp,
            network,
            master_key: ExtendedPrivKey::new_master(network, &seed).unwrap(),
        }
    }

    pub fn new_bitcoin_receive_address(&self, account_index: u32, address_index: u32) -> Address {
        let change: u32 = 0;
        let coin_type = 0;
        let path = format!("m/44'/{coin_type}'/{account_index}'/{change}/{address_index}");
        let derivation_path = DerivationPath::from_str(&path).unwrap();
        // Derive the child private key
        let child_key = self
            .master_key
            .derive_priv(&self.secp, &derivation_path)
            .unwrap();
        let private_key = child_key.private_key;
        let public_key =
            PublicKey::from_str(private_key.public_key(&self.secp).to_string().as_str()).unwrap();
        //   let receive_address = Address::p2pkh(&public_key, self.network);

        Address::p2wpkh(&public_key, self.network).unwrap()
    }

    pub fn new_bitcoin_receive_qrcode(
        bitcoin_address: Address,
        amount: &str,
        label: &str,
        message: &str,
    ) -> String {
        let uri = format!(
            "bitcoin:{}?amount={}&label={}&message={}",
            bitcoin_address, amount, label, message
        );
        uri
    }
}
