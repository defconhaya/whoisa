use std::io::Read;
use std::time::Duration;
use std::{io::Write, net::TcpStream};


pub fn whois(server_addr:&str, ip:&str)->String{
    // let query = format!("-B {}\r\n", ip); //ripe
    let query = format!("n + {}\r\n", ip); //arin
    // let query = "?\r\n";
    let mut client =  TcpStream::connect(server_addr).unwrap();
    let _=client.set_read_timeout(Some(Duration::from_secs(1)));
    let _=client.set_write_timeout(Some(Duration::from_secs(1)));
    let _=client.write_all(query.as_bytes());
    let _=client.flush();
    let mut buffer = Vec::new();
    let _response = client.read_to_end(&mut buffer);
    let query_result = String::from_utf8_lossy(&buffer);
    query_result.to_string()
    }