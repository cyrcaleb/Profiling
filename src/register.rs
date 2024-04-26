use super::universal_machine::{UniversalMachine, Segment};
use std::io::{stdin, stdout, Read, Write};

impl UniversalMachine {
    /// Conditional move: If the value in register `reg1` is not zero, copy the value from register `reg3` to register `reg2`.
    pub fn cmov(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        if self.get_register(reg1 as usize) != 0 {
            self.set_register(reg2 as usize, self.get_register(reg3 as usize));
        }
    }
    
    /// Load: Load the value at memory address represented by the value in register `reg2` and offset `reg3` into register `reg1`.
    pub fn load(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        self.set_register(reg1 as usize, self.get_val_from_memory_space(self.get_register(reg2 as usize), self.get_register(reg3 as usize)));
    }
    
    /// Store: Store the value in register `reg3` at the memory address represented by the value in register `reg1` and offset `reg2`.
    pub fn store(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        self.set_val_from_memory_space(self.get_register(reg1 as usize), self.get_register(reg2 as usize), self.get_register(reg3 as usize));
    }
    
    /// Add: Add the values in registers `reg2` and `reg3`, and store the result in register `reg1`.
    pub fn add(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        self.set_register(reg1 as usize, self.get_register(reg2 as usize).wrapping_add(self.get_register(reg3 as usize)));
    }
    
    /// Multiply: Multiply the values in registers `reg2` and `reg3`, and store the result in register `reg1`.
    pub fn mul(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        self.set_register(reg1 as usize, self.get_register(reg2 as usize).wrapping_mul(self.get_register(reg3 as usize)));
    }
    
    /// Divide: Divide the value in register `reg2` by the value in register `reg3`, and store the result in register `reg1`.
    /// Panics if attempting to divide by zero.
    pub fn div(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        if self.get_register(reg3 as usize) == 0 { panic!("Cannot Divide By 0") }
        self.set_register(reg1 as usize, self.get_register(reg2 as usize).wrapping_div(self.get_register(reg3 as usize)));
    }
    
    /// NAND: Bitwise NAND of the values in registers `reg2` and `reg3`, and store the result in register `reg1`.
    pub fn nand(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = self.get_register(reg2 as usize);
        let reg3_value = self.get_register(reg3 as usize);
    
        self.set_register(reg1 as usize, !(reg2_value & reg3_value) as u32);
    }
    
    /// Halt: Terminate the program.
    pub fn halt(&mut self) {
        std::process::exit(0);
    }
    
    // INVARIANT: Segments will always be placed within a free location. If there are no free locations, it will be
    // inserted in a new location in the address space.
    /// Map segment: Map a new memory segment of size specified by the value in register `reg2`. 
    /// The address of the mapped segment is stored in register `reg1`.
    pub fn mapseg(&mut self, reg1: u32, reg2: u32) {
        if self.get_free_memory_len() > 0 {
            let num_words = self.get_register(reg2 as usize);
            let segment = Segment { data: vec![0; num_words as usize] };
            self.set_segment_from_memory_space(self.get_from_free_memory(), segment);
            self.set_register(reg1 as usize, self.get_from_free_memory());
            self.pop_free_memory();
            return;
        }
        let num_words = self.get_register(reg2 as usize);
        let segment = Segment { data: vec![0; num_words as usize] };
        self.push_memory_space(segment);
        self.set_register(reg1 as usize, (self.get_memory_space_len()-1) as u32);
    }
    
    // INVARIANT: For every element that undergoes unmap, the location of that element will always be freed, but
    // will not shift or modify the address space
    /// Unmap segment: Unmap the memory segment at the address specified by the value in register `reg1`.
    pub fn unmapseg(&mut self, reg1: u32) {
        let segment = Segment { data: vec![0] };
        self.set_segment_from_memory_space(self.get_register(reg1 as usize), segment);
        self.push_free_memory(self.get_register(reg1 as usize));
    }
    
    /// Output: Output the ASCII character corresponding to the value in register `reg1`.
    pub fn output(&mut self, reg1: u32) {
        let reg_value = self.get_register(reg1 as usize) as u8;
        stdout().write_all(&[reg_value]).unwrap();
        stdout().flush().unwrap();
    }
    
    /// Input: Read an ASCII character from input and store its value in register `reg1`.
    pub fn input(&mut self, reg1: u32) {
        let mut input_buffer = [0_u8; 1]; // Buffer to store the input byte
        if let Ok(n) = stdin().read(&mut input_buffer) {
            if n > 0 { // If a byte was successfully read
                let input_value = input_buffer[0] as u32;
                self.set_register(reg1 as usize, input_value);
            } else {
                eprintln!("Error: No input bytes read.");
            }
        } else {
            eprintln!("Error: Failed to read input.");
        }
    }
    
    /// Load program: Load the program segment specified by the value in register `reg2`.
    /// If `reg1` is non-zero, clear the segment at address 0 before loading the program.
    pub fn loadprog(&mut self, reg1: u32, reg2: u32) {
        if self.get_register(reg1 as usize) == 0 {
            self.set_program_counter(self.get_register(reg2 as usize));
        } else {
            let segment = self.get_segment_from_memory_space(self.get_register(reg1 as usize));
            self.set_segment_from_memory_space(0, segment);
            self.set_program_counter(self.get_register(reg2 as usize));
        }
    }
    
    /// Load value: Load a constant value into register `reg1`.
    pub fn loadval(&mut self, reg1: u32, value: u32) {
        self.set_register(reg1 as usize, value);
    }
} 