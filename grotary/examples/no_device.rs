use std::net::TcpStream;

use grotary::run;


fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 10];

    run(&mut stream, data, recv)?;
    println!("recv: {recv:?}");
    Ok(())
}