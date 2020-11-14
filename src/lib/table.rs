use std::cmp::Ordering;

#[derive(Debug)]
pub struct Table {
    table: Vec<i8>,
    page_count: i32,
}

impl Table {
    fn check_page(&self, page_num: i32) {
        let to_compare = self.page_count - 1; // indexed from 0
        match page_num.cmp(&to_compare) {
            Ordering::Greater => panic!("page number too big"),
            _ => dbg!(format!("requested page {} of {}, OK!", page_num, self.page_count))
        };
    }

    pub fn init(page_count: i32) -> Table {
        let mut table: Vec<i8> = Vec::new();
        table.resize(page_count as usize, 0);
        Table { table, page_count }
    }

    pub fn size(&self) -> i32 {
        self.page_count
    }

    pub fn get_page(&self, page_num: i32) -> i8 {
        self.check_page(page_num);
        self.table[page_num as usize]
    }

    /// probably useless :(
    fn set_flags(&mut self, page_num: i32, r: i8, m: i8) {
        if (r != 1 && r!= 0) || (m != 2 && m!= 0) {
            dbg!(format!("r = {}, m = {}", r, m));
            panic!("incorrect bit flags");
        }
        self.check_page(page_num);
        self.table.insert(page_num as usize, r + m);
    }

    pub fn set_read(&mut self, page_num: i32) {
        self.check_page(page_num);
        match self.table[page_num as usize] {
            1 | 3 => (), // do not modify
            2 => self.table.insert(page_num as usize, 3), // add read flag
            _ => self.table.insert(page_num as usize, 0)  // reset corrupted
        }
    }

    pub fn set_write(&mut self, page_num: i32) {
        self.check_page(page_num);
        match self.table[page_num as usize] {
            2 | 3 => (), // do not modify
            1 => self.table.insert(page_num as usize, 3), // add write flag
            _ => self.table.insert(page_num as usize, 0) // reset corrupted
        }
    }
}

pub struct TableIterator<'a> {
    table: &'a Table,
    pos: usize
}

impl TableIterator<'_> {
    pub fn get_for(table: &Table) -> TableIterator {
        TableIterator {table, pos: 0}
    }
}

impl Iterator for TableIterator<'_> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.table.table.len() {
            None
        } else {
            self.pos += 1;
            Some(self.table.table[self.pos - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Table;

    #[test]
    fn test_init() {
        let table = self::Table::init(1024);
        assert_eq!(table.size(), 1024)
    }

    #[test]
    #[should_panic]
    fn test_page_gt_size() {
        let table = self::Table::init(1024);
        table.get_page(1024);
    }

    #[test]
    fn test_page_eq_size() {
        let table = self::Table::init(1024);
        table.get_page(1023);
    }

    #[test]
    fn test_page_lt_size() {
        let table = self::Table::init(1024);
        table.get_page(1022);
    }

    #[test]
    #[should_panic]
    fn test_set_flags_page_too_big() {
        let mut table = self::Table::init(1024);
        table.set_flags(1024, 0, 0)
    }

    #[test]
    #[should_panic]
    fn test_set_flags_incorrect_flags() {
        let mut table = self::Table::init(1024);
        table.set_flags(0, 2, 1)
    }

    #[test]
    fn test_set_flag_r() {
        let mut table = self::Table::init(1024);
        table.set_flags(0, 1, 0);
        assert_eq!(1, table.get_page(0));
    }

    #[test]
    fn test_set_flag_m() {
        let mut table = self::Table::init(1024);
        table.set_flags(0, 0, 2);
        assert_eq!(2, table.get_page(0));
    }

    #[test]
    fn test_set_flag_rm() {
        let mut table = self::Table::init(1024);
        table.set_flags(0, 1, 2);
        assert_eq!(3, table.get_page(0));
    }
}
