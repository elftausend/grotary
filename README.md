# grotary

Send, for instance, live sensor data to a (g)rotary server and process the data there. Afterwards, the result is sent back to the client.

The data always passes through an untrained / random generated neural network. However, this step should be customizable, which is not the case at the moment.

## Server

Main function used in grotary-server:
Set the "ROTARY_ADDRESS" environment variable to an ip:port combination you want.
```rust
pub use server::RotaryServer;

fn main() -> Result<(), std::io::Error> {
    let address = std::env::var("ROTARY_ADDRESS").unwrap_or("127.0.0.1:12000".into());
    println!("Try binding the TCP socket: {address}");
    RotaryServer::bind(address)?;
    Ok(())
}
```

## [Client]
[Client]: https://github.com/elftausend/grotary/tree/main/grotary

```rust
use grotary::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new(0, "127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 10];
    
    device.run(data, recv)?;
    Ok(())
}
```