use crate::lib::table::Table;
use crate::lib::vm::VirtualMemory;
use rand::Rng;
use std::cmp::Ordering;
use crate::lib::nru::NRU;

mod lib {
    pub mod table;
    pub mod nru;
    pub mod vm;
}

fn seed(vm: &mut VirtualMemory) {
    let mut rng = rand::thread_rng();
    let pages_count = vm.pages_count();
    let page_size = vm.page_size();
    for _ in 1 .. pages_count {
        let page = rng.gen_range(0, pages_count);
        let on_page = rng.gen_range(0, page_size);
        let addr = page as i64 * page_size as i64 + on_page as i64;
        match rng.gen_range(0, 3).cmp(&1) {
            Ordering::Less => {
                vm.write(addr, 1);
            }
            Ordering::Equal => {
                vm.write(addr, 1);
                vm.read(addr);
            }
            Ordering::Greater => {
                vm.read(addr);
            }
        }
    }
}

fn main() {
    let mut vm = VirtualMemory::init(10, 16);
    for _ in 1 .. 3 {
        vm.reset();
        seed(&mut vm);
    }
    let mut nru = NRU::init();
    nru.go_through(&vm);
    nru.dbg_print_stats();
    println!("Page {} chosen", nru.get_nru_page());
}
