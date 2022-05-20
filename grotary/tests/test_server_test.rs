use grotary::Device;

#[test]
fn test_server_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new(0, "127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 28*28];
    
    device.run(data, recv)?;
    println!("recv: {recv:?}");
    Ok(())
}