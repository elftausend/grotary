use std::{net::TcpStream, io::{Write, Read}};

pub fn to_bytes(data: &[f32]) -> Vec<u8> {
    let mut bytes = vec![0; data.len() * 4];

    for (idx, value) in data.iter().enumerate() {
        let start = idx * 4;
        bytes[start..start+4].copy_from_slice(&value.to_le_bytes())
    }
    bytes
}

pub fn from_bytes(buf: &[u8]) -> Vec<f32> {
    let mut data = vec![0.; buf.len() / 4];

    for i in 0..data.len() {
        data[i] = f32::from_le_bytes(buf[i*4..i*4+4].try_into().unwrap());
    }
    data
}

#[test]
fn connect() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;

    let mut send = vec![0u8; 8 + 2];
    send[..8].copy_from_slice(&2u64.to_le_bytes());
    send[8..].copy_from_slice(&[1, 1]);
    stream.write(&send)?;

    for _ in 0..100000 {
        let mut send = (28*28*4u64+1).to_le_bytes().to_vec();
        send.extend(vec![2]);
        send.extend(to_bytes(&[0.241f32; 28*28]));
        
        stream.write(&send)?;
    
        let mut recv = [0; 10*4];
        stream.read(&mut recv)?;
    
        let result = from_bytes(&recv);
        //println!("result: {result:?}");    
        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    
    /* 
    let bytes: u64 = 5;
    
    let mut send = vec![0u8; bytes as usize + 8];
    send[..8].copy_from_slice(&bytes.to_le_bytes());
    send[8..].copy_from_slice(&[1, 5, 5, 3, 2]);
    stream.write(&send)?;
    stream.write(&send)?;
    stream.write(&send)?;
    
    let data = [2; 28*28*4];
    let mut send = vec![0u8; data.len() + 8];
    send[..8].copy_from_slice(&data.len().to_le_bytes());
    send[8..].copy_from_slice(&data);

    
    stream.write(&send)?;
    
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    for _ in 0..500 {
        stream.write(&send)?;

        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
    */
    
    Ok(())
}