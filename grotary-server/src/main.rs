mod layer_impl;
mod convert;
mod device;

use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use convert::{from_bytes, to_bytes};

use custos::{Matrix, set_count, Device};
use device::RotaryDevice;
use gradients::{Linear, Softmax, ReLU};
use layer_impl::Network;

fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");

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
    let mut device: RotaryDevice = Default::default();

    let mut network = Network::new();

    let mut data = [0; 5000000];
            
    let mut bytes_to_read = 0;
    let mut old_data = Vec::new();

    let mut bytes;
    loop {  
        match stream.read(&mut data) {
            Ok(n) => {
                if n == 0 {
                    break
                }
                //println!("read {n} bytes");
                let mut start = 0;
    
                while start < n {
                    if bytes_to_read > 0 {
                        let mut packet = vec![0; old_data.len()+bytes_to_read];
                        packet[0..old_data.len()].copy_from_slice(&old_data);
                        packet[old_data.len()..].copy_from_slice(&data[0..bytes_to_read]);
                        
                        
                        start = bytes_to_read;
                        bytes_to_read = 0;

                        handle_packet(&packet, &mut network, &mut device, &mut stream);

                    } else {
                        bytes = u64::from_le_bytes(data[start..start+8].try_into().unwrap());

                        let bound = start + bytes as usize + 8;

                        if bound > n {
                            bytes_to_read = bound - n;
                            old_data = data[start+8..bytes as usize + start+8].to_vec();
                            println!("next packet: partially empty, bytes to read: {}", bytes_to_read);

                        }
                        //println!("packet len: {bytes}");
                        let packet = &data[start+8..bytes as usize + start+8];
                        handle_packet(packet, &mut network, &mut device, &mut stream);
                        start = bound;
                    }
                    
                }
                //println!("start: {start}, read_bytes: {n}");
                
            },
            Err(e) => {
                println!("e: {e}");
                break
            },
        }   
    }       
}           

fn handle_packet(packet: &[u8], network: &mut Network, device: &mut RotaryDevice, stream: &mut TcpStream) {
    let id = packet[0];
    match id {
        1 => {
            *device = packet[1].into();
            *network = Network::from_layers(
                vec![
                    Box::new(Linear::new(784, 128)),
                    Box::new(ReLU::new()),
                    Box::new(Linear::new(128, 10)),
                    Box::new(ReLU::new()),
                    Box::new(Linear::new(10, 10)),
                    Box::new(Softmax::new()),
        
                ]  
            );
            println!("device: {device:?}");
        }

        // receive client data, network forward pass -> send result to client
        2 => {
            let forward = from_bytes(&packet[1..]);
            let features = 784;

            let samples = forward.len() / features;
            let forward = Matrix::from((samples, features, forward));

            let output = to_bytes(&network.forward(forward).read());
            set_count(0);
            
            match &mut device.opencl {
                Some(cl) => {
                    cl.drop(forward.to_buf());
                },
                None => {
                    device.cpu.as_mut().unwrap().drop(forward.to_buf());
                }
            }
    
            stream.write_all(&output).unwrap();            
        }
        _ => {}
    }        
}