use std::net::TcpStream;
use std::env;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::thread;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let port = &args[2];
    let mut writer = TcpStream::connect(format!("{}:{}", ip, port))?;
    let mut reader = BufReader::new(writer.try_clone()?);

    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            print!("{}", buf);
        }
    });

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                writer.write(line.as_bytes())?;
                writer.write(&[b'\n'])?;
            },
            Err(_) => panic!("asd"),
        };
    }
    //stream.write("nnldnsdknlsd".as_bytes())?;


    Ok(())
}

fn listen_thread() {

}
