use std::sync::{Arc, Mutex};

use crossbeam::channel::Sender;
use tokio::runtime::Runtime;
use tonic::{
    async_trait, server::NamedService, transport::Server, Request, Response, Result, Status,
};
use whoosh::{bitcoin_service_server::BitcoinService, ExchangeRequest, PaymentUriReply, Quote};

use crate::{cache::Cache, indexer::ReceiveAddress, masterkey::MasterKey};

pub mod whoosh {
    tonic::include_proto!("whoosh");
}

struct MyBitcoinService {
    cache: Arc<Mutex<Cache>>,
    receive_address_sender: Sender<ReceiveAddress>,
}

impl MyBitcoinService {
    pub fn new(
        cache: Arc<Mutex<Cache>>,
        receive_address_sender: Sender<ReceiveAddress>,
    ) -> MyBitcoinService {
        MyBitcoinService {
            cache,
            receive_address_sender,
        }
    }
}

#[async_trait]
impl BitcoinService for MyBitcoinService {
    async fn generate(
        &self,
        request: Request<ExchangeRequest>,
    ) -> Result<Response<PaymentUriReply>, Status> {
        let req = request.get_ref();
        let receive_address;
        let cache_ref = self.cache.clone();
        {
            if let Result::Ok(mut cache) = cache_ref.lock() {
                receive_address = cache.generate_payment_receive().unwrap();
                self.receive_address_sender.send(receive_address.clone());
            } else {
                let status =
                    Status::internal("Internal server error , failed to access the cache!");
                return Err(status);
            }
        }

        let message = format!(
            "Bridge {} BTC to {}",
            &req.source_token_amount, &req.target_chain
        );
        let payment_uri = MasterKey::new_bitcoin_receive_qrcode(
            receive_address.address.clone(),
            req.source_token_amount.clone().as_str(),
            "WHOOSH Bridge!",
            message.as_str(),
        );
        let reply = PaymentUriReply { payment_uri };

        return Ok(Response::new(reply));
    }

    async fn get_quote(
        &self,
        request: Request<ExchangeRequest>,
    ) -> Result<Response<Quote>, Status> {
        let req = request.get_ref();
        let quote = Quote {
            estimated_target_token_amount: "100".to_string(),
            target_chain: "ethereum".to_string(),
        };

        return Ok(Response::new(quote));
    }
}

impl NamedService for MyBitcoinService {
    const NAME: &'static str = "MyBitcoin Service";
}

pub fn run_gateway(cache: Arc<Mutex<Cache>>, receive_address_sender: Sender<ReceiveAddress>) {
    let addr = "[::1]:50051".parse().unwrap();
    let server = MyBitcoinService::new(cache, receive_address_sender);
    println!("Bitcoin service listening on {}", addr);
    let rt = Runtime::new().expect("failed to obtain a new RunTime object");
    let server_future = Server::builder()
        .add_service(whoosh::bitcoin_service_server::BitcoinServiceServer::new(
            server,
        ))
        .serve(addr);
    rt.block_on(server_future);
}
