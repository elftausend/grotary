use grotary_server::RotaryServer;

#[test]
fn test_server() -> Result<(), std::io::Error> {
    RotaryServer::bind("127.0.0.1:12000", |x| {
        x
    })?;
    
    Ok(())
}


        
