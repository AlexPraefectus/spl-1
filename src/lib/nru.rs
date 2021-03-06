use crate::lib::vm::VirtualMemory;
use crate::lib::table::TableIterator;
use rand::Rng;

pub struct NRU {
    class0: Vec<i32>,
    class1: Vec<i32>,
    class2: Vec<i32>,
    class3: Vec<i32>,
}

fn dbg_print_class(name: &str, cls: &Vec<i32>){
    println!("{}", format!("{} - {} pages total", name, cls.len()));
    println!("{}", format!("{} pages: {:?}", name, cls));
}

impl NRU {
    pub fn init() -> NRU {
      NRU {
          class0: Vec::new(),
          class1: Vec::new(),
          class2: Vec::new(),
          class3: Vec::new(),
      }
    }

    pub fn dbg_print_stats(&self) {
        println!("{:=<80}", "");
        println!("{:~<35}NRU stats{:~<36}", "", "");
        dbg_print_class("Class 0", &self.class0);
        dbg_print_class("Class 1", &self.class1);
        dbg_print_class("Class 2", &self.class2);
        dbg_print_class("Class 3", &self.class3);
        println!("{:=<80}", "");
    }

    pub fn go_through(&mut self, vm: &VirtualMemory) {
        self.class0.clear();
        self.class1.clear();
        self.class2.clear();
        self.class3.clear();
        for (idx, flags) in TableIterator::get_for(vm.get_table()).enumerate(){
            match flags {
                0 => self.class0.push(idx as i32),
                1 => self.class1.push(idx as i32),
                2 => self.class2.push(idx as i32),
                3 => self.class3.push(idx as i32),
                _ => ()
            }
        }

    }

    pub fn get_nru_page(&mut self) -> i32{
        let mut rng = rand::thread_rng();
        let retval: i32;

        if !self.class0.is_empty() {
            let idx = rng.gen_range(0, self.class0.len());
            retval = self.class0[idx];
        } else if !self.class1.is_empty() {
            let idx = rng.gen_range(0, self.class1.len());
            retval = self.class1[idx];
        } else if !self.class2.is_empty() {
            let idx = rng.gen_range(0, self.class2.len());
            retval = self.class2[idx];
        } else {
            let idx = rng.gen_range(0, self.class3.len());
            retval = self.class3[idx];
        }

        return retval;
    }
}