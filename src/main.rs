use crate::lib::table::Table;

mod lib {
    pub mod table;
    pub mod nru;
}

fn main() {
    let table = Table::init(1024);
    println!("{:?}", table);
    table.get_page(1024);
    println!("Hello, world!");
}
