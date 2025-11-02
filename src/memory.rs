pub struct Memory {
    pub data: Box<[u8; 8192]>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: Box::new([0; 8192]),
        }
    }
}
