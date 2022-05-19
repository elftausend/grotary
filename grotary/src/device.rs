use std::{net::{TcpStream, ToSocketAddrs}, io::{Write, Read}};

use crate::{to_bytes, from_bytes, DeviceIDError};

pub struct Device {
    pub stream: TcpStream
}

impl Device {
    pub fn new<A: ToSocketAddrs>(id: u8, addr: A) -> Result<Device, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(addr)?;

        let mut send = [0u8; 8 + 2];
        send[..8].copy_from_slice(&2u64.to_le_bytes());
        send[8..].copy_from_slice(&[1, id]);
        stream.write_all(&send)?;

        let mut success = [0];
        stream.read_exact(&mut success)?;

        if success[0] == 0 {
            return Err(Box::new(DeviceIDError));
        }

        Ok(Device {
            stream
        })
    }

    pub fn run(&mut self, data: &[f32], recv: &mut [f32]) -> Result<(), std::io::Error> {
        let mut send = (data.len() as u64 *4+1).to_le_bytes().to_vec();
        send.extend(vec![2]);
        send.extend(to_bytes(data));
        
        self.stream.write_all(&send)?;

        let mut bytes_recv = [0; 10*4];
        self.stream.read_exact(&mut bytes_recv)?;
        from_bytes(&bytes_recv, recv);
        Ok(())
    }
}
