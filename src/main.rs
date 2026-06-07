use crate::cpu::{Cpu, CpuError};
use crate::memory::Memory;
use std::{env::args, fs::File, io::Read};
pub mod cpu;
pub mod memory;
fn main() {
    let mut cpu = Cpu::new();
    let mut args = args().skip(1);
    let path = args.next().expect("provide a file name");
    for reg in args {
        assert_eq!(reg.as_bytes()[0], b'x');
        let r: usize = String::from_utf8(Vec::from(&reg.as_bytes()[1..=2]))
            .expect("provide register number")
            .parse()
            .unwrap();
        assert_eq!(reg.as_bytes()[3], b'=');
        let v: u64 = String::from_utf8(Vec::from(&reg.as_bytes()[4..]))
            .expect("provide register value")
            .parse()
            .unwrap();
        cpu.regs[r] = v;
    }
    cpu.regs[0] = 0;
    let mut file = Vec::new();
    File::open(path)
        .expect("path is invalid")
        .read_to_end(&mut file)
        .unwrap();
    cpu.mem.load(file.as_slice(), 0);
    loop {
        let word = u32::from_le_bytes(cpu.mem.read_word(cpu.pc));
        let opcode: u8 = word as u8 & 0b01111111;
        let iimm = (word as i32 >> 20) as i32;
        let iimmf = (word >> 20);
        let rs1 = ((word >> 15) & 0b11111) as usize;
        let rs2 = ((word >> 20) & 0b11111) as usize;
        let funct3 = ((word >> 12) & 0b111) as u8;
        let funct7 = ((word >> 25) & 0b1111111) as u8;
        let funct6 = ((word >> 26) & 0b111111) as u8;
        let uimm = ((word >> 20) & 0b111111) as u8;
        let rd = ((word >> 7) & 0b11111) as usize;
        let imm20 = word >> 12;
        match cpu.circle() {
            Ok(_) => {}
            Err((CpuError::Unimplemented, addr)) => {
                panic!(
                    "unimplemented instruction \"0x{:x}\" on address: 0x{:x}",
                    u32::from_le_bytes(cpu.mem.read_word(addr)),
                    addr
                );
            }
            Err((CpuError::WrongFormat, addr)) => {
                panic!(
                    "wrong instruction \"0x{:x}\" on address: 0x{:x}",
                    u32::from_le_bytes(cpu.mem.read_word(addr)),
                    addr
                );
            }
            Err((CpuError::End, _)) => {
                break;
            }
        }
    }
    for x in 0..32 {
        println!("x{}: {}", x, cpu.regs[x])
    }
}
