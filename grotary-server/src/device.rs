use custos::{InternCPU, InternCLDevice, CLDevice, CPU, AsDev};

#[allow(dead_code)]
#[derive(Debug)]
pub struct RotaryDevice {
    pub cpu: Option<InternCPU>,
    pub opencl: Option<InternCLDevice>
}

impl Default for RotaryDevice {
    fn default() -> Self {
        Self { cpu: None, opencl: None }
    }
}


impl From<u8> for RotaryDevice {
    fn from(id: u8) -> Self {
        println!("id: {id}");
        if id == 0 {
            RotaryDevice {cpu: Some(CPU::new().select()), opencl: None }
        } else {
            RotaryDevice {
                cpu: None, 
                opencl: Some(CLDevice::get(id as usize-1).unwrap().select()) 
            }
        }
    }
}
