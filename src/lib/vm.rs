use super::table::Table;
use std::cmp::Ordering;

pub struct VirtualMemory {
    memory: Vec<Vec<i8>>,
    table: Table,
    page_size: i8,
}

impl VirtualMemory {
    pub fn init(page_count: i32, page_size: i8) -> VirtualMemory {
        let mut memory: Vec<Vec<i8>> = Vec::new();
        let mut placeholder: Vec<i8> = Vec::new();
        placeholder.resize(page_size as usize, 0);
        memory.resize(page_count as usize, placeholder.clone());
        let table = Table::init(page_count);
        VirtualMemory { memory, table, page_size }
    }

    /// check for address out of memory/page bounds
    fn check_addr(&self, addr: i64) {
        let max_page_idx = self.memory.len() as i64 - 1; // indexed from 0
        let to_compare = max_page_idx * self.page_size as i64;
        match addr.cmp(&to_compare) {
            Ordering::Greater => panic!("addr too big"),
            _ => dbg!(format!("requested addr {} of {}, OK!", addr, to_compare))
        };
    }

    /// add 1 to flag if needed
    pub fn read(&mut self, addr: i64) -> i8 {
        self.check_addr(addr);
        let page = addr / self.page_size as i64;
        let local_addr = addr - (page * self.page_size as i64);
        self.table.set_read(page as i32);
        self.memory[page as usize][local_addr as usize]
    }

    /// add 2 to flags if needed
    pub fn write(&mut self, addr: i64, value: i8) {
        self.check_addr(addr);
        let page = addr / self.page_size as i64;
        let local_addr = addr - (page * self.page_size as i64);
        self.table.set_write(page as i32);
        self.memory[page as usize].insert(local_addr as usize, value);
    }

    /// reset statistics
    pub fn reset(&mut self) {
        self.table = Table::init(self.memory.len() as i32)
    }

    pub fn get_table(&self) -> &Table {
        &self.table
    }
}
