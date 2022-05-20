use grotary::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new(0, "127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 10];
    
    device.run(data, recv)?;
    Ok(())
}