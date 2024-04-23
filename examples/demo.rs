use rsemu::{
    bus::bus::Bus,
    cpu::{riscv::RiscV32, CPU},
    devices::ram::Ram,
};

fn main() {
    println!("hello from rsemu examples!");
    let mut ram = Ram::new(0x100000);
    ram.from_file("/Project/test.bin");

    let mut bus = Bus::new();
    bus.add_device(Box::new(ram), 0x80000000, 0x90000000);

    let mut cpu = RiscV32::new();
    cpu.pc = 0x80000000;
    loop {
        cpu.step(&mut bus);
    }
}
