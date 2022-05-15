use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12000")?;
    listener.set_nonblocking(true)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
        
    }

    Ok(())

}

fn handle_client(mut stream: TcpStream) {
    let mut data = Vec::<u8>::with_capacity(1000);

    loop {
        match stream.read_to_end(&mut data) {
            Ok(n) => {
                println!("read {n} bytes");
            },
            Err(_) => todo!(),
        }
    }
}
