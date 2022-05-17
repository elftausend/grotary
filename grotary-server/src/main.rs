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
            
    //let mut network = Network::new();
    let mut wait_for_data = false;
    let mut bytes_to_read = 0;
    let mut old_data = Vec::new();

    let mut bytes = 0;
    loop {  
        match stream.read(&mut data) {
            Ok(n) => {
                if n == 0 {
                    break
                }
                println!("read {n} bytes");
                let mut start = 0;
    
                while start < n {

                    if wait_for_data {
                        let mut packet = vec![0; old_data.len()+bytes_to_read];
                        packet[0..old_data.len()].copy_from_slice(&old_data);
                        packet[old_data.len()..].copy_from_slice(&data[0..bytes_to_read]);
                        
                        let id = packet[0];
                        match id {
                            1 => {
                                println!("1 id");
                            }
                            2 => {
                                println!("2 id sers");
                            }
                            _ => {}
                        }    
                        bytes_to_read = 0;
                        wait_for_data = false;
                        start = bytes_to_read;

                    } else {
                        // read_bytes smaller than start?
                        // => diff of start-read_bytes: packet after read_bytes is empty -> get diff bytes from next packets
                        bytes = u64::from_le_bytes(data[start..start+8].try_into().unwrap());

                        let bound = start + bytes as usize + 8;

                        if bound > n {
                            bytes_to_read = bound - n;
                            wait_for_data = true;
                            old_data = data[start+8..bytes as usize + start+8].to_vec();
                            //println!("next packet: partially empty, bytes to read: {}", bytes_to_read);

                        }
                        //println!("packet len: {bytes}");
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
                        start = bound;
                    }
                    
                }
                println!("start: {start}, read_bytes: {n}");
            
//                process_packet_id(&data, n);
                
            },
            Err(e) => {
                println!("e: {e}");
                break
            },
        }   
    }       
}           

fn process_packet_id(data: &[u8], read_bytes: usize) {
    let mut start = 0;
    
    while start < read_bytes {
        // read_bytes smaller than start?
        // => diff of start-read_bytes: packet after read_bytes is empty -> get diff bytes from next packets
        let bytes = u64::from_le_bytes(data[start..start+8].try_into().unwrap());
        println!("packet len: {bytes}");
        let packet = &data[start+8..bytes as usize + start+8];
        let id = packet[0];
        match id {
            1 => {
                println!("1 id");
            }
            2 => {
    //            println!("2 id");
            }
            _ => {}
        }    
        start += bytes as usize + 8;
    }
    println!("start: {start}, read_bytes: {read_bytes}");
}
