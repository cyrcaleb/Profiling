use crate::rumdis;

pub struct UniversalMachine {
    pub registers: Vec<u32>,
    pub program_counter: u32,
    pub memory_space: Vec<Vec<u32>>,
    pub free_memory: Vec<u32>,
} 

impl UniversalMachine {
    pub unsafe fn new(instructions: Vec<u32>) -> Self {
        let mut registers = Vec::with_capacity(8);
        registers.resize(8, 0);  
        let memory_space = vec![instructions]; 
        let program_counter = 0;
        let free_memory = Vec::new();

        UniversalMachine {
            registers,
            program_counter,
            memory_space,
            free_memory,
        }
    }

    pub unsafe fn get_segment_from_memory_space(&self, address: u32) -> &[u32] {
        self.memory_space.get_unchecked(address as usize)
    }

    pub unsafe fn set_segment_from_memory_space(&mut self, address: u32, value: Vec<u32>) {
        if (address as usize) < self.memory_space.len() {
            self.memory_space[address as usize] = value;
        }
    }

    pub unsafe fn get_val_from_memory_space(&self, address: u32, offset: u32) -> u32 {
        let segment = self.memory_space.get_unchecked(address as usize);
        *segment.get_unchecked(offset as usize)
    }

    pub unsafe fn set_val_from_memory_space(&mut self, address: u32, offset: u32, value: u32) {
        let segment = self.memory_space.get_unchecked_mut(address as usize);
        *segment.get_unchecked_mut(offset as usize) = value;
    }

    pub unsafe fn get_from_free_memory(&self) -> u32 {
        self.free_memory[self.free_memory.len() - 1]
    }

    pub unsafe fn run(&mut self) {
        loop {
            let instruction = self.get_val_from_memory_space(0, self.program_counter);
            rumdis::disassemble(instruction, self);
        }
    }
}
