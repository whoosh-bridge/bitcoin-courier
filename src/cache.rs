extern crate redis;

use anyhow::Ok;
use redis::{Client, Commands, Connection};
use serde_json::json;

use crate::{enums::NetworkType, indexer::ReceiveAddress, masterkey::MasterKey};

pub struct Cache{
  client: Client,
  connection: Connection,
  masterkey:  MasterKey,
  account_index: u32,
  address_index: u32,
}

const MAX_ADDRESSES: u32 = 2^31;


impl Cache{
  pub fn initialize(network: NetworkType,addr: String)->anyhow::Result<Cache>{
    let seed =*  b"Qu/E,.qp40ruLCX8GDSYlE2m?:I[|}5,";
    let client = redis::Client::open(format!("redis://{}",addr))?;
    let mut connection = client.get_connection()?;
    let account_index = connection.get("LAST_ACCOUNT_INDEX").unwrap_or(0u32);
    let address_index = connection.get("LAST_ADDRESS_INDEX").unwrap_or(0u32);
    // con.set("LAST_ACCOUNT_INDEX", account_index).context("Failed to read LAST_ACCOUNT_INDEX");
    // con.set("LAST_ADDRESS_INDEX", address_index).context("Failed to read LAST_ADDRESS_INDEX");

    redis::cmd("SET").arg("LAST_ACCOUNT_INDEX").arg(account_index).exec(&mut connection)?;
    redis::cmd("SET").arg("LAST_ADDRESS_INDEX").arg(address_index).exec(&mut connection)?;    
    
    let bitcoin_network = match network {
      NetworkType::Testnet => nakamoto::common::bitcoin::Network::Testnet,
      NetworkType::Mainnet =>  nakamoto::common::bitcoin::Network::Bitcoin
    };
    let masterkey = MasterKey::new(seed, bitcoin_network);

    return Ok(Cache{
      masterkey,
      client,
      connection,
      account_index,
      address_index,
    })
  }

  pub fn generate_payment_receive(&mut self)-> anyhow::Result<ReceiveAddress>{        

    let address = self.masterkey.new_bitcoin_receive_address(self.account_index, self.address_index);
    let script = address.script_pubkey().clone();
    
    self.address_index = self.address_index + 1;
    if self.address_index >= MAX_ADDRESSES {
      self.account_index += 1;
      self.address_index = 0;
    }

    let mut pipeline = redis::pipe();
    pipeline.cmd("SET").arg("LAST_ACCOUNT_INDEX").arg(self.account_index);
    pipeline.cmd("SET").arg("LAST_ADDRESS_INDEX").arg(self.address_index);
    

    let receive_address = ReceiveAddress{
      script: script.clone(),
      address,
      account_index: self.account_index.clone(),
      address_index: self.address_index.clone()
    };
    let k= format!("RECEIVE_ADDRESS_{}",script.to_string());
    let v = json!(receive_address).to_string();
    pipeline.cmd("SET").arg(k.clone()).arg(v);
    pipeline.cmd("EXPIRE").arg(k).arg(3600); // The generated address is valid for 1 hour , 3600 seconds

    pipeline.exec(&mut self.connection)?;
    
    Ok(receive_address)
  }

}