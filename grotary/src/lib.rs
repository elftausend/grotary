mod convert;
mod device;

use std::{fmt::Display, net::TcpStream, io::{Write, Read}};

pub use convert::*;
pub use device::*;

#[derive(Debug)]
pub struct DeviceIDError;

impl Display for DeviceIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DeviceIDError {}

pub fn run(stream: &mut TcpStream, data: &[f32], recv: &mut [f32]) -> Result<(), std::io::Error> {
    let mut send = (data.len() as u64 *4+1).to_le_bytes().to_vec();
    send.extend(vec![2]);
    send.extend(to_bytes(data));
    
    stream.write_all(&send)?;

    let mut bytes_recv = vec![0; recv.len() * 4];
    stream.read_exact(&mut bytes_recv)?;
    from_bytes(&bytes_recv, recv);
    Ok(())    
}