mod convert;
mod device;

use std::fmt::Display;

pub use convert::*;
pub use device::*;

#[derive(Debug)]
pub struct DeviceIDError;

impl Display for DeviceIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DeviceIDError {}