use std::{net::TcpStream, io::{Write, Read}};

use grotary::{Device, to_bytes, from_bytes};

#[test]
fn connect_with_device() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new(0, "127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 10];
    
    device.run(data, recv)?;

    Ok(())
}

#[test]
fn connect_with_device_for() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new(1, "127.0.0.1:12000")?;

    let data = &[0.4312; 28*28];
    let recv = &mut [0.; 10];
    
    for _ in 0..1000 {
        device.run(data, recv)?;
    }
    
    Ok(())
}

#[test]
fn test_invalid_id() -> Result<(), Box<dyn std::error::Error>> {
    match Device::new(255, "127.0.0.1:12000") {
        Ok(_) => {
            panic!("should not find device");    
        },
        Err(e) => {
            if e.to_string() != "DeviceIDError" {
                panic!("should not find device");
            }
        },
    }

    
    Ok(())
}

#[test]
fn connect() -> Result<(), std::io::Error> {

    /*
    const TIMES: usize = 10000;

    let mut s = 0;

    let before = Instant::now();
    for _ in 0..TIMES {
        let mut send = (28*28*4u64+1).to_le_bytes().to_vec();
        send.extend(vec![2]);
        send.extend(to_bytes(&[0.241f32; 28*28]));
        s += send[0];
    }
    let after = Instant::now();
    println!("s: {s} duration: {:?}", (after-before) / TIMES as u32);
    */
     
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;

    let mut send = vec![0u8; 8 + 2];
    send[..8].copy_from_slice(&2u64.to_le_bytes());
    send[8..].copy_from_slice(&[1, 0]);
    stream.write(&send)?;

    for _ in 0..10000000 {
        let mut send = (28*28*4u64+1).to_le_bytes().to_vec();
        send.extend(vec![2]);
        send.extend(to_bytes(&[0.241f32; 28*28]));
        
        stream.write(&send)?;
    
        let mut recv = [0; 10*4];
        stream.read(&mut recv)?;
    
        let mut rx = [0.; 10];
        from_bytes(&recv, &mut rx);
        //println!("result: {result:?}");    
        //std::thread::sleep(std::time::Duration::from_millis(1));
    }
    
    
    
    /* 
    let bytes: u64 = 5;
    
    let mut send = vec![0u8; bytes as usize + 8];
    send[..8].copy_from_slice(&bytes.to_le_bytes());
    send[8..].copy_from_slice(&[1, 5, 5, 3, 2]);
    stream.write(&send)?;
    stream.write(&send)?;
    stream.write(&send)?;
    
    let data = [2; 28*28*4];
    let mut send = vec![0u8; data.len() + 8];
    send[..8].copy_from_slice(&data.len().to_le_bytes());
    send[8..].copy_from_slice(&data);

    
    stream.write(&send)?;
    
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    for _ in 0..500 {
        stream.write(&send)?;

        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
    */
    
    Ok(())
}