use tonic::{server::NamedService, transport::Server, Request, Response, Status,async_trait};
use whoosh::{bitcoin_service_server::BitcoinService, ExchangeRequest, PaymentUriReply, Quote};
use tokio::runtime::Runtime;


pub mod whoosh {
    tonic::include_proto!("whoosh");
}

#[derive(Debug, Default)]
struct MyBitcoinService{}

#[async_trait]
impl  BitcoinService for MyBitcoinService {
    async fn generate(&self,request: Request<ExchangeRequest>,) ->  Result<Response<PaymentUriReply>, Status> {
        let reply = PaymentUriReply{
            payment_uri: "".to_string()
        };
        return Ok(Response::new(reply))
    }

    async fn get_quote(&self,request: Request<ExchangeRequest> ,) ->  Result<Response<Quote> ,Status> {
        let req = request.get_ref();
        let quote = Quote{
            estimated_target_token_amount: "100".to_string(),
            target_chain: "ethereum".to_string()
        }; 
        
        return Ok(Response::new(quote))
    }
}

impl NamedService for MyBitcoinService{
    const NAME: &'static str = "MyBitcoin Service";
}

fn run_gateway(){
    let addr = "[::1]:50051".parse().unwrap();
    let server = MyBitcoinService::default();
    println!("Bitcoin service listening on {}", addr);
    let rt = Runtime::new().expect("failed to obtain a new RunTime object");
    let server_future = Server::builder()
        .add_service(whoosh::bitcoin_service_server::BitcoinServiceServer::new(server) )
        .serve(addr);
    rt.block_on(server_future);
}