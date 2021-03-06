use custos::{CPU, AsDev, Matrix, set_count};
use gradients::{Linear, ReLU, Softmax};
use grotary_server::{RotaryServer, Network};

/* 
fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let address = std::env::var("ROTARY_ADDRESS").unwrap_or("127.0.0.1:12000".into());
    println!("Try binding the TCP socket: {address}");
    RotaryServer::bind(address,  |x| {
        x
    })?;
    Ok(())
}
*/
fn main() -> Result<(), std::io::Error> {
    let mut a = 0;


    let mut net = Network::default();
    RotaryServer::new("127.0.0.1:12000")
        .init(move ||  {
            a = 3;
            /* 
            net = Network::from_layers(
                vec![
                    Box::new(Linear::new(784, 128)),
                    Box::new(ReLU::new()),
                    Box::new(Linear::new(128, 10)),
                    Box::new(ReLU::new()),
                    Box::new(Linear::new(10, 10)),
                    Box::new(Softmax::new()),
                ]);
                */
        })
        .exec(|x| {

            a = 3;

            let features = 784;
        
            let samples = x.len() / features;
            let forward = Matrix::from((samples, features, x.clone()));

            net.forward(forward);
            set_count(0);
            
            //device.drop(forward.to_buf());
            x
        })
        .bind()?;


    /*RotaryServer::bind("127.0.0.1:12000", move |x| {
        /*
        let features = 784;
        
        let _samples = x.len() / features;
        let forward = Matrix::from((samples, features, x));

        net.forward(forward);
        set_count(0);
        
        device.drop(forward.to_buf());
        forward.read()
        */
        x
    })?;*/
    
    Ok(())
}


        
