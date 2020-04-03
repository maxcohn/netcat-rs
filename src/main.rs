use std::net::Ipv4Addr;
use std::net::TcpStream;
use std::env;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let port = &args[2];
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;

    stream.write("nnldnsdknlsd".as_bytes())?;


    Ok(())
}
