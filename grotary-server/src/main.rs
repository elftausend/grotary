mod layer_impl;

use std::{net::{TcpListener, TcpStream}, io::Read};



fn main() -> Result<(), std::io::Error> {
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
    let mut data = Vec::<u8>::with_capacity(1000);

    loop {
        match stream.read_to_end(&mut data) {
            Ok(n) => {
                if n == 0 {
                    break
                }

                println!("read {n} bytes");
            },
            Err(e) => {
                println!("e: {e}");
                break
            },
        }
    }
}
