use std::io::Result;

fn main() -> Result<()> {          
    tonic_build::configure()
    .build_server(true)    
    .build_client(true)    
    .build_transport(true)
    .compile_protos(&["protos/whoosh.proto",], &["protos/"])?;
    Ok(())
}