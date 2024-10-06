mod indexer;
// mod args;
mod keygen;
// mod enums;

use anyhow::Result;

use indexer::run_indexer;

// async fn app(args: Args) -> Result<()> {
    
//     let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
//     let listener = TcpListener::bind(addr).await?;
//     println!("Listening on http://{}", addr);
//     loop{
//         // When an incoming TCP connection is received grab a TCP stream for
//         // client<->server communication.
//         //
//         // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
//         // .await point allows the Tokio runtime to pull the task off of the thread until the task
//         // has work to do. In this case, a connection arrives on the port we are listening on and
//         // the task is woken up, at which point the task is then put back on a thread, and is
//         // driven forward by the runtime, eventually yielding a TCP stream.
//         let (tcp, _) = listener.accept().await?;
//         // Use an adapter to access something implementing `tokio::io` traits as if they implement
//         // `hyper::rt` IO traits.
//         let io = TokioIo::new(tcp);

//         // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
//         // current task without waiting for the processing of the HTTP1 connection we just received
//         // to finish
//         tokio::task::spawn(async move {
//             // Handle the connection from the client using HTTP1 and pass any
//             // HTTP requests received on that connection to the `hello` function
//             if let Err(err) = http1::Builder::new()
//                 .timer(TokioTimer::new())
//                 .serve_connection(io, service_fn(hello))
//                 .await
//             {
//                 println!("Error serving connection: {:?}", err);
//             }
//         });
//     };

//     // tracing_subscriber::fmt::init();

//     // let addr = std::net::SocketAddr::new(args.address.parse()?, args.port);

//     // let todos = Todos::new();
//     // let context = Arc::new(RwLock::new(todos));

//     // serve(addr, context, handle).await?;
//     Ok(())
// }



fn main() -> Result<()> {
    // let args = Args::parse();
    
    run_indexer()?;
    // let indexer = thread::spawn(||{
    //     run_indexer();
    // });
    
    // tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async {
    //         app(args).await;
    //     });
    
    // indexer.join().expect("Couldn't join on the associated thread");;
    Ok(())
}




