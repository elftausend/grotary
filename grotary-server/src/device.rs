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

impl RotaryDevice {
    pub fn new(id: u8) -> Result<RotaryDevice, Box<dyn std::error::Error>> {
        if id == 0 {
            Ok(RotaryDevice {cpu: Some(CPU::new().select()), opencl: None })
        } else {
            Ok(RotaryDevice {
                cpu: None, 
                opencl: Some(CLDevice::get(id as usize-1)?.select()) 
            })
        }
    }
}


impl From<u8> for RotaryDevice {
    fn from(id: u8) -> Self {
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
