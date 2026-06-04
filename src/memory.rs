use std::collections::HashMap;
use std::ops::{Index, IndexMut};
pub struct Memory {
    ram: HashMap<u64, [u8; PAGE_SIZE as usize]>,
}
pub const PAGE_SIZE: u64 = 4096;
impl Index<u64> for Memory {
    type Output = u8;
    fn index(&self, index: u64) -> &u8 {
        let page = index / PAGE_SIZE;
        let offset = (index % PAGE_SIZE) as usize;
        if self.ram.contains_key(&page) {
            return &self.ram[&page][offset];
        } else {
            return &0;
        }
    }
}
impl IndexMut<u64> for Memory {
    fn index_mut(&mut self, index: u64) -> &mut u8 {
        let page = index / PAGE_SIZE;
        let offset = (index % PAGE_SIZE) as usize;
        self.init_page(page);
        return self.ram.get_mut(&page).unwrap().get_mut(offset).unwrap();
    }
}
impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: HashMap::new(),
        }
    }
    fn init_page(&mut self, page: u64) {
        if !self.ram.contains_key(&page) {
            self.ram.insert(page, [0u8; PAGE_SIZE as usize]);
        }
    }
    pub fn read_word(&self, addr: u64) -> [u8; 4] {
        let mut r = [0u8; 4];
        for i in 0..4 {
            r[i] = self[addr + i as u64]
        }
        r
    }
    pub fn write_word(&mut self, addr: u64, word: [u8; 4]) {
        for i in 0..4 {
            self[addr + i] = word[i as usize];
        }
    }
    pub fn load(&mut self, buf: &[u8], addr: u64) {
        for (offset, byte) in buf.iter().enumerate() {
            self[addr + offset as u64] = *byte;
        }
    }
}
