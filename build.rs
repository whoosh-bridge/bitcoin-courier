use std::io::Result;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {          
    tonic_build::configure()
    .build_server(true)    
    .build_client(true)    
    .build_transport(true)
    .compile(&["protos/whoosh.proto",], &["protos/"])?;
    Ok(())
}