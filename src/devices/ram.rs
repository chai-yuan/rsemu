use crate::devices::Device;
use std::fs::File;
use std::io::Read;

pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Ram {
            memory: vec![0; size],
        }
    }

    pub fn from_file(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        file.read_to_end(&mut self.memory).unwrap();
    }
}

impl Device for Ram {
    fn read(&mut self, address: u64, size: u8) -> u64 {
        let mut value = 0u64;
        for i in 0..size {
            let byte = self.memory[address as usize + i as usize];
            value |= (byte as u64) << (i * 8);
        }
        value
    }

    fn write(&mut self, address: u64, size: u8, data: u64) {
        for i in 0..size {
            let byte = ((data >> (8 * i)) & 0xFF) as u8;
            self.memory[address as usize + i as usize] = byte;
        }
    }
}
