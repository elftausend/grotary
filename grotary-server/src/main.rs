use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:12000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())

}

fn handle_client(mut stream: TcpStream) {
    let mut data = Vec::<u8>::with_capacity(1000);

    loop {
        match stream.read_to_end(&mut data) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
