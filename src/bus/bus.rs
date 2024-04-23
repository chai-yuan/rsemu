use crate::devices::Device;

struct DeviceWithRange {
    device: Box<dyn Device>, // 使用Box来存放具体设备
    start_address: u64,      // 设备的起始地址
    end_address: u64,        // 设备的结束地址（不包括此地址）
}

pub struct Bus {
    devices: Vec<DeviceWithRange>, // 存放所有的设备及其地址范围
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>, start_address: u64, end_address: u64) {
        self.devices.push(DeviceWithRange {
            device,
            start_address,
            end_address,
        });
    }

    pub fn read(&mut self, address: u64, size: u8) -> u64 {
        for d in &mut self.devices {
            if address >= d.start_address && address < d.end_address {
                return d.device.read(address - d.start_address, size);
            }
        }
        panic!("No device found for address {}", address);
    }

    pub fn write(&mut self, address: u64, size: u8, data: u64) {
        for d in &mut self.devices {
            if address >= d.start_address && address < d.end_address {
                d.device.write(address - d.start_address, size, data);
                return;
            }
        }
        panic!("No device found for address {}", address);
    }
}
