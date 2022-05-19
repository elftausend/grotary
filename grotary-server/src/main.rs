mod layer_impl;
mod convert;
mod device;
mod server;

pub use server::RotaryServer;


fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");

    RotaryServer::bind("127.0.0.1:12000")?;
    Ok(())
}


        
