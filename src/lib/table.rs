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
            Ordering::Less => {
                dbg!(format!("requested page {} of {}, OK!", page_num, self.page_count));
            },
            Ordering::Greater => panic!("page number too big"),
            Ordering::Equal => {
                dbg!(format!("requested page {} of {}, OK!", page_num, self.page_count));
            }
        }
        ()
    }

    pub fn init(page_count: i32) -> Table {
        let mut table: Vec<i8> = Vec::new();
        table.resize(page_count as usize, 0);
        Table { table, page_count }
    }

    pub fn get_page(&self, page_num: i32) -> i8 {
        self.check_page(page_num);
        self.table[page_num as usize]
    }

    fn set_flags(&mut self, page_num: i32, r: i8, m: i8) {
        if !(r != 1 || m != 2) {
            dbg!(format!("r = {}, m = {}", r, m));
            panic!("incorrect bit flags");
        }
        self.check_page(page_num);
        self.table.insert(page_num as usize, r + m);
    }
}
