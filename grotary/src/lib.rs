use std::{net::TcpStream, io::Write};


#[test]
fn connect() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;
    let bytes: u64 = 5;
    
    let mut send = vec![0u8; bytes as usize + 8];
    send[..8].copy_from_slice(&bytes.to_le_bytes());
    send[8..].copy_from_slice(&[1, 5, 5, 3, 2]);
    stream.write(&send)?;
    stream.write(&send)?;
    
    
    let data = [2; 10000];
    let mut send = vec![0u8; data.len() + 8];
    send[..8].copy_from_slice(&data.len().to_le_bytes());
    send[8..].copy_from_slice(&data);

    stream.write(&send)?;
    stream.write(&send)?;

    stream.write(&send)?;
    stream.write(&send)?;
    stream.write(&send)?;
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    stream.write(&send)?;
    stream.write(&send)?;
    stream.write(&send)?;
    Ok(())
}