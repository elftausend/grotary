pub fn to_bytes(data: &[f32]) -> Vec<u8> {
    let mut bytes = vec![0; data.len() * 4];

    for (idx, value) in data.iter().enumerate() {
        let start = idx * 4;
        bytes[start..start+4].copy_from_slice(&value.to_le_bytes())
    }
    bytes
}

pub fn from_bytes(buf: &[u8]) -> Vec<f32> {
    let mut data = vec![0.; buf.len() / 4];

    for i in 0..data.len() {
        data[i] = f32::from_le_bytes(buf[i*4..i*4+4].try_into().unwrap());
    }
    data
}