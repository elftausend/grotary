use grotary_server::RotaryServer;

#[test]
fn test_server() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let address = std::env::var("ROTARY_ADDRESS").unwrap_or("127.0.0.1:12000".into());
    println!("Try binding the TCP socket: {address}");
    RotaryServer::bind(address)?;
    Ok(())
}


        
