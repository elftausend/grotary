use std::{net::TcpStream, io::Write};


#[test]
fn connect() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;
    let data = vec![1; 100000];
    stream.write(&data)?;
    stream.write(&data)?;
    stream.write(&data)?;
    stream.write(&data)?;
    stream.write(&data)?;
    stream.write(&data)?;
    stream.write(&data)?;
    Ok(())
}