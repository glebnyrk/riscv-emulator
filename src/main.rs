use std::{env::args, fs::File, io::Read};

use crate::memory::Memory;

pub mod memory;
struct Cpu {
    mem: Memory,
    regs: [u64; 32],
    pc: u64,
}
fn main() {
    let mut cpu = Cpu {
        mem: Memory::new(),
        regs: [0u64; 32],
        pc: 0,
    };
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
        let rs1 = ((word >> 15) & 0b11111) as usize;
        let rs2 = ((word >> 20) & 0b11111) as usize;
        let funct3 = ((word >> 12) & 0b111) as u8;
        let funct7 = ((word >> 25) & 0b1111111) as u8;
        let rd = ((word >> 7) & 0b11111) as usize;
        match opcode {
            0b0010011 => {
                //OP-IMM
                match funct3 {
                    0b000 => {
                        //ADDI
                        cpu.regs[rd] = (cpu.regs[rs1] as i64).wrapping_add(iimm as i64) as u64;
                    }
                    0b010 => {
                        //SLTI
                        cpu.regs[rd] = if (cpu.regs[rs1] as i64) < iimm as i64 {
                            1
                        } else {
                            0
                        }
                    }
                    0b011 => {
                        //SLTIU
                        cpu.regs[rd] = if (cpu.regs[rs1]) < iimm as u64 { 1 } else { 0 };
                    }
                    _ => unimplemented!(),
                }
            }
            0b0110011 => {
                //OP
                match (funct3, funct7) {
                    (0b000, 0b000000) => {
                        //ADD
                        cpu.regs[rd] = cpu.regs[rs1].wrapping_add(cpu.regs[rs2]);
                    }
                    _ => unimplemented!(),
                }
            }

            0 => {
                break;
            }
            _ => unimplemented!(),
        }
        cpu.regs[0] = 0;
        cpu.pc += 4;
    }
    for x in 0..32 {
        println!("x{}: {}", x, cpu.regs[x])
    }
}
