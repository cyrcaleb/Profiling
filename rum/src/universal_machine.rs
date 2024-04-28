use crate::rumdis;

pub struct UniversalMachine {
    pub registers: [u32; 8],
    pub program_counter: u32,
    pub memory_space: Vec<Vec<u32>>,
    pub free_memory: Vec<u32>,
}

impl UniversalMachine {
    pub unsafe fn new(instructions: Vec<u32>) -> Self {
        let registers = [0; 8];
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

    pub unsafe fn get_val_from_memory_space(&self, address: u32, offset: u32) -> u32 {
        let segment = self.memory_space.get_unchecked(address as usize);
        *segment.get_unchecked(offset as usize)
    }

    pub unsafe fn set_val_from_memory_space(&mut self, address: u32, offset: u32, value: u32) {
        if let Some(segment) = self.memory_space.get_mut(address as usize) {
            *segment.get_unchecked_mut(offset as usize) = value;
        }
    }

    pub unsafe fn run(&mut self) {
        loop {
            let instruction = self.get_val_from_memory_space(0, self.program_counter);
            rumdis::disassemble(instruction, self);
        }
    }
}
