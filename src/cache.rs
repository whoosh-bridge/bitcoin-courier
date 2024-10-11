extern crate redis;

use anyhow::Ok;
use redis::{Client, Commands, Connection};

use crate::{enums::NetworkType, masterkey::MasterKey};

struct Cache{
  client: Client,
  connection: Connection,
  masterkey:  MasterKey,
  account_index: u32,
  address_index: u32,
}

impl Cache{
  pub fn initialize(network: NetworkType)->anyhow::Result<Cache>{
    let seed =*  b"Qu/E,.qp40ruLCX8GDSYlE2m?:I[|}5,";
    let client = redis::Client::open("redis://156.255.1.32:6379")?;
    let mut connection = client.get_connection()?;
    let mut account_index = connection.get("LAST_ACCOUNT_INDEX").unwrap_or(0u32);
    let mut address_index = connection.get("LAST_ADDRESS_INDEX").unwrap_or(0u32);
    // con.set("LAST_ACCOUNT_INDEX", account_index).context("Failed to read LAST_ACCOUNT_INDEX");
    // con.set("LAST_ADDRESS_INDEX", address_index).context("Failed to read LAST_ADDRESS_INDEX");

    redis::cmd("SET").arg("LAST_ACCOUNT_INDEX").arg(account_index).exec(&mut connection)?;
    redis::cmd("SET").arg("LAST_ADDRESS_INDEX").arg(address_index).exec(&mut connection)?;    
    
    let bitcoin_network = match network {
      NetworkType::Testnet => nakamoto::common::bitcoin::Network::Testnet,
      NetworkType::Mainnet =>  nakamoto::common::bitcoin::Network::Bitcoin
    };
    let masterkey = MasterKey::new(seed, bitcoin_network);
    masterkey.new_bitcoin_receive_address(0, 0);

    return Ok(Cache{
      masterkey,
      client,
      connection,
      account_index,
      address_index,
    })
  }

  pub fn generate_payment_receive(&mut self)-> anyhow::Result<()>{    
    let account_index = self.account_index;
    let address_index = self.address_index;    

    let receive_address = self.masterkey.new_bitcoin_receive_address(account_index, address_index);
    let watchlist_address = receive_address.script_pubkey().clone();
    
    redis::cmd("SET").arg("LAST_ACCOUNT_INDEX").arg(account_index).exec(&mut self.connection)?;
    redis::cmd("SET").arg("LAST_ADDRESS_INDEX").arg(address_index).exec(&mut self.connection)?;

    Ok(())
  }

}