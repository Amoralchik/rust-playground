use crate::http::Request;
use std::{convert::TryFrom, io::Read, net::TcpListener};

pub struct Server {
    addr: String,
    port: u32,
}

impl Server {
    pub fn new(addr: String, port: u32) -> Server {
        Server { addr, port }
    }

    pub fn run(self) {
        let address: String = format!("{}:{}", self.addr, self.port);
        println!("Listening on {}", &address);
        let listener: TcpListener = TcpListener::bind(&address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("stream: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Failed: {}", e)
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed: {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed: {}", e)
                }
            }
        }
    }
}
