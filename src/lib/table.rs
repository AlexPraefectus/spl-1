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

    fn set_flags(&mut self, page_num: i32, r: i8, m: i8) {
        if (r != 1 && r!= 0) || (m != 2 && m!= 0) {
            dbg!(format!("r = {}, m = {}", r, m));
            panic!("incorrect bit flags");
        }
        self.check_page(page_num);
        self.table.insert(page_num as usize, r + m);
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
