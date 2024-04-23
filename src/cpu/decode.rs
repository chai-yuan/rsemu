// 一些方便译码的函数

pub fn extract_bits(instruction: u32, start: u32, end: u32) -> u32 {
    let mask = (1 << (end - start + 1)) - 1;
    (instruction >> start) & mask
}

pub fn sign_extend(value: u32, bit_size: u32) -> i32 {
    let shift = 32 - bit_size;
    ((value << shift) as i32) >> shift
}

// 方便译码的宏

macro_rules! decode_sign_imm {
    ($instr:expr, $start:expr, $end:expr) => {{
        let value = extract_bits($instr, $start, $end);
        sign_extend(value, $end - $start)
    }};
}
