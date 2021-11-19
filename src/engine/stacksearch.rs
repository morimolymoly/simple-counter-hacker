use std::collections::HashMap;

use memflow::*;
use memflow_win32::*;

struct Target<T> {
    address: Address,
    value: T,
}

pub struct StackSearch<T: VirtualMemory> {
    stack_base: Address,
    stack_limit: Address,
    process: Win32Process<T>,
    target_u32: HashMap<u64, u32>,
}

impl<T: VirtualMemory> StackSearch<T> {
    pub fn new(stack_base: Address, stack_limit: Address, process: Win32Process<T>) -> Self {
        StackSearch {
            stack_base,
            stack_limit,
            process,
            target_u32: HashMap::new(),
        }
    }

    pub fn search_u32(&mut self, target: u32) -> usize {
        let mut address = self.stack_base;
        if self.target_u32.is_empty() {
            loop {
                if address < self.stack_limit {
                    break;
                }
    
                let val = self.process.virt_mem.virt_read::<u32>(address).unwrap();
                if val == target {
                    self.target_u32.insert(address.as_u64(), val);
                } else {
                    if self.target_u32.contains_key(&address.as_u64()) {
                        self.target_u32.remove(&address.as_u64());
                    }
                }
                address -= 4;
            }
        } else {
            for address in self.target_u32.clone().keys().clone().collect::<Vec<_>>() {
                let value = self.process.virt_mem.virt_read::<u32>(Address::from(*address)).unwrap();
                if value != target {
                    self.target_u32.remove(address).unwrap();
                }
            }
        }
        for address in self.target_u32.clone().keys().clone().collect::<Vec<_>>() {
            println!("{:x}", address);
        }
        return self.target_u32.len();
    }

    pub fn cheat(&mut self, value: i32) {
        let addr = self.target_u32.keys().clone().collect::<Vec<_>>();
        if addr.len() == 1 {
            self.process.virt_mem.virt_write(Address::from(*addr[0]), &value).unwrap();
        } else {
            println!("もっと探せ！！！");
        }
    }
}