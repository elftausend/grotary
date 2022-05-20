use std::{net::{TcpListener, ToSocketAddrs, TcpStream}, io::{Write, Read}};

use custos::opencl::api::OCLErrorKind;

use crate::{device::RotaryDevice, convert::{to_bytes, from_bytes}};

pub struct RotaryServer;

impl RotaryServer {
    pub fn bind<A: ToSocketAddrs, F: Fn(Vec<f32>) -> Vec<f32> + Copy>(addr: A, exec: F) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle_client(stream, exec),
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
        Ok(())
    }

    
}

fn handle_client<F: Fn(Vec<f32>) -> Vec<f32> + Copy>(mut stream: TcpStream, exec: F) {
    let mut device: RotaryDevice = Default::default();

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

                let mut start = 0;
    
                while start < n {
                    if bytes_to_read > 0 {
                        let mut packet = vec![0; old_data.len()+bytes_to_read];
                        packet[0..old_data.len()].copy_from_slice(&old_data);
                        packet[old_data.len()..].copy_from_slice(&data[0..bytes_to_read]);
                        
                        
                        start = bytes_to_read;
                        bytes_to_read = 0;

                        handle_packet(&packet, &mut device, &mut stream, exec);

                    } else {
                        bytes = u64::from_le_bytes(data[start..start+8].try_into().unwrap());

                        let bound = start + bytes as usize + 8;

                        if bound > n {
                            bytes_to_read = bound - n;
                            old_data = data[start+8..bytes as usize + start+8].to_vec();

                        }

                        let packet = &data[start+8..bytes as usize + start+8];
                        handle_packet(packet, &mut device, &mut stream, exec);
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

fn handle_packet<F: Fn(Vec<f32>) -> Vec<f32>>(
    packet: &[u8], 
    device: &mut RotaryDevice, 
    stream: &mut TcpStream,
    exec: F,
) {
    let id = packet[0];
    match id {
        1 => {
            //*device = packet[1].into();
            let mut success = 1;
            match RotaryDevice::new(packet[1]) {
                Ok(dev) => *device = dev,
                Err(e) => {
                    if e.kind() == Some(&OCLErrorKind::InvalidDeviceIdx) {
                        success = 0;                 
                    }
                },
            }

            /* 
            if success == 1 {
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
            }
            */ 
            stream.write_all(&[success]).unwrap();
        }

        // receive client data, network forward pass -> send result to client
        2 => {

            let process = from_bytes(&packet[1..]);
            let output = to_bytes(&exec(process));

            /* 
            let forward = from_bytes(&packet[1..]);
            let features = 784;

            let samples = forward.len() / features;
            let forward = Matrix::from((samples, features, forward));

            let output = to_bytes(&network.forward(forward).read());
            set_count(0);
            
            device.drop_buf(forward.to_buf());
            */
    
            stream.write_all(&output).unwrap();            
        }
        _ => {}
    }        
}