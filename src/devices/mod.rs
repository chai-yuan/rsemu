pub mod ram;

pub trait Device {
    // 读取数据
    fn read(&mut self, address: u64, size: u8) -> u64;
    // 写入数据
    fn write(&mut self, address: u64, size: u8, data: u64);
}
