mod layer_impl;

use std::{net::{TcpListener, TcpStream}, io::Read};

use gradients::{Linear, Softmax};
use layer_impl::Network;



fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => panic!("encountered IO error: {}", e),
        }
        
    }

    Ok(())

}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 500000];

    let mut network = Network::new();

    loop {
        match stream.read(&mut data) {
            Ok(n) => {
                if n == 0 {
                    break
                }
                println!("read {n} bytes");
                process_packet_id(&data, n);
                
            },
            Err(e) => {
                println!("e: {e}");
                break
            },
        }
        data = [0; 500000];
    }
}

fn process_packet_id(data: &[u8], read_bytes: usize) {
    let mut start = 0;
    
    while start != read_bytes {
        let bytes = u64::from_le_bytes(data[start..start+8].try_into().unwrap());
        println!("packet len: {bytes}");
        let packet = &data[start+8..bytes as usize + start+8];
        let id = packet[0];
        match id {
            1 => {
                println!("1 id");
            }
            2 => {
                println!("2 id");
            }
            _ => {}
        }
        
        start += bytes as usize + 8;
    }
    

    

    
}
