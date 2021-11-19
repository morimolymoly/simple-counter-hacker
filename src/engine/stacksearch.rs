use memflow::*;
use memflow_win32::*;

pub struct StackSearch<T> {
    stack_base: Address,
    stack_limit: Address,
    process: Win32Process<T>,

    target_u64: Vec<u64>,
}

impl<T> StackSearch<T> {
    pub fn new(stack_base: Address, stack_limit: Address, process: Win32Process<T>) -> Self {
        StackSearch {
            stack_base,
            stack_limit,
            process,

            target_u64: Vec::new(),
        }
    }

    pub fn search_u64(&mut self, target: u64) {
    }

    fn search() {
    }
}