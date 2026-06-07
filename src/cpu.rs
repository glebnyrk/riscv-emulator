use crate::memory::Memory;
pub struct Cpu {
    pub mem: Memory,
    pub regs: [u64; 32],
    pub pc: u64,
}
#[derive(Debug)]
pub enum CpuError {
    ///Instruction is not yet either implemented or just doesn't exist
    Unimplemented,
    ///Wrong instruction format (for SRLI, SRAI, SLLI)
    WrongFormat,
    //End of executable code (0x0000 in test behaviour)
    End,
}
impl Cpu {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            regs: [0; 32],
            pc: 0,
        }
    }
    pub fn circle(&mut self) -> Result<(), (CpuError, u64)> {
        let word = u32::from_le_bytes(self.mem.read_word(self.pc));
        match self.run(word) {
            Ok(_) => {}
            Err(e) => return Err((e, self.pc)),
        }
        self.pc = self.pc.wrapping_add(4);
        return Ok(());
    }
    pub fn run(&mut self, word: u32) -> Result<(), CpuError> {
        match opcode(word) {
            0b0010011 => self.op_imm(word),
            0b0110011 => self.op(word),
            0b0110111 => self.lui(word),
            0 => return Err(CpuError::End),
            _ => return Err(CpuError::Unimplemented),
        }?;
        self.regs[0] = 0;
        return Ok(());
    }
    pub fn op_imm(&mut self, word: u32) -> Result<(), CpuError> {
        let rs1 = self.regs[rs1(word)];
        let iimm = iimm(word);
        let iimmf = iimmf(word);
        let rd = &mut self.regs[rd(word)];
        match funct3(word) {
            0b000 => {
                //ADDI
                *rd = (rs1 as i64).wrapping_add(iimm as i64) as u64;
            }
            0b010 => {
                //SLTI
                *rd = if (rs1 as i64) < iimm as i64 { 1 } else { 0 }
            }
            0b011 => {
                //SLTIU
                *rd = if (rs1) < iimm as u64 { 1 } else { 0 };
            }
            0b111 => {
                *rd = rs1 & iimmf as u64;
            }
            0b110 => {
                //ORI
                *rd = rs1 | iimmf as u64;
            }
            0b001 => {
                //SLLI
                match funct6(word) {
                    0b000000 => {
                        *rd = rs1 << uimm(word);
                    }
                    _ => return Err(CpuError::WrongFormat),
                }
            }
            0b101 => {
                match funct6(word) {
                    0b000000 => {
                        //SRLI
                        *rd = rs1 >> uimm(word);
                    }
                    0b010000 => {
                        //SRAI
                        *rd = (rs1 as i64 >> uimm(word)) as u64;
                    }
                    _ => return Err(CpuError::WrongFormat),
                }
            }
            0b100 => {
                //XORI
                *rd = rs1 ^ iimmf as u64
            }
            _ => unreachable!(),
        };
        return Ok(());
    }
    fn op(&mut self, word: u32) -> Result<(), CpuError> {
        let rs1 = self.regs[rs1(word)];
        let rs2 = self.regs[rs2(word)];
        let rd = &mut self.regs[rd(word)];
        match (funct3(word), funct7(word)) {
            (0b000, 0b0000000) => {
                //ADD
                *rd = rs1.wrapping_add(rs2);
            }
            (0b000, 0b0100000) => {
                //SUB
                *rd = rs1.wrapping_sub(rs2);
            }
            (0b001, 0b0000000) => {
                //SLL
                *rd = rs1 << rs2;
            }
            (0b010, 0b0000000) => {
                //SLT
                *rd = if (rs1 as i64) < (rs2 as i64) { 1 } else { 0 };
            }
            (0b011, 0b0000000) => {
                //SLTU
                *rd = if rs1 < rs2 { 1 } else { 0 };
            }
            (0b100, 0b0000000) => {
                //XOR
                *rd = rs1 ^ rs2;
            }
            (0b101, 0b0000000) => {
                //SRL
                *rd = rs1 >> rs2;
            }
            (0b101, 0b0100000) => {
                //SRA
                *rd = (rs1 as i64 >> rs2) as u64;
            }
            (0b110, 0b0000000) => {
                //OR
                *rd = rs1 | rs2;
            }
            (0b111, 0b0000000) => {
                //AND
                *rd = rs1 & rs2;
            }

            _ => return Err(CpuError::Unimplemented),
        }
        return Ok(());
    }
    pub fn lui(&mut self, word: u32) -> Result<(), CpuError> {
        self.regs[rd(word)] = (word & 0xFFFFF000) as u64;
        return Ok(());
    }
}
fn opcode(word: u32) -> u8 {
    word as u8 & 0b1111111
}
fn funct3(word: u32) -> u8 {
    ((word >> 12) & 0b111) as u8
}
fn funct6(word: u32) -> u8 {
    ((word >> 26) & 0b111111) as u8
}
fn funct7(word: u32) -> u8 {
    ((word >> 25) & 0b1111111) as u8
}
fn rs1(word: u32) -> usize {
    ((word >> 15) & 0b11111) as usize
}
fn rs2(word: u32) -> usize {
    ((word >> 20) & 0b11111) as usize
}
fn rd(word: u32) -> usize {
    ((word >> 7) & 0b11111) as usize
}
fn iimm(word: u32) -> i32 {
    (word as i32 >> 20) as i32
}
fn iimmf(word: u32) -> u32 {
    word >> 20
}
fn uimm(word: u32) -> u8 {
    ((word >> 20) & 0b111111) as u8
}
