use super::universal_machine::{UniversalMachine, Segment};
use std::io::{stdin, stdout, Read, Write};

impl UniversalMachine {
    
    pub unsafe fn cmov(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg1_value = *self.registers.get_unchecked(reg1 as usize);
        if reg1_value != 0 {
            *self.registers.get_unchecked_mut(reg2 as usize) = *self.registers.get_unchecked(reg3 as usize);
        }
    }
    
    pub unsafe fn load(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        *self.registers.get_unchecked_mut(reg1 as usize) = self.get_val_from_memory_space(reg2_value, reg3_value);
    }
    
    pub unsafe fn store(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg1_value = *self.registers.get_unchecked(reg1 as usize);
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        self.set_val_from_memory_space(reg1_value, reg2_value, reg3_value);
    }

    pub unsafe fn add(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        *self.registers.get_unchecked_mut(reg1 as usize) = reg2_value.wrapping_add(reg3_value);
    }
    
    pub unsafe fn mul(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        *self.registers.get_unchecked_mut(reg1 as usize) = reg2_value.wrapping_mul(reg3_value);
    }
    
    pub unsafe fn div(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        if reg3_value == 0 { panic!("Cannot Divide By 0") }
        *self.registers.get_unchecked_mut(reg1 as usize) = reg2_value.wrapping_div(reg3_value);
    }
    
    pub unsafe fn nand(&mut self, reg1: u32, reg2: u32, reg3: u32) {
        let reg2_value = *self.registers.get_unchecked(reg2 as usize);
        let reg3_value = *self.registers.get_unchecked(reg3 as usize);
        *self.registers.get_unchecked_mut(reg1 as usize) = !(reg2_value & reg3_value);
    }
    
    pub unsafe fn halt(&mut self) {
        std::process::exit(0);
    }
    
    pub unsafe fn mapseg(&mut self, reg1: u32, reg2: u32) {
        let free_memory_len = self.get_free_memory_len();
        if free_memory_len > 0 {
            let num_words = *self.registers.get_unchecked(reg2 as usize);
            let segment = Segment { data: vec![0; num_words as usize] };
            self.set_segment_from_memory_space(self.get_from_free_memory(), segment);
            *self.registers.get_unchecked_mut(reg1 as usize) = self.get_from_free_memory();
            self.pop_free_memory();
            return;
        }
        let num_words = *self.registers.get_unchecked(reg2 as usize);
        let segment = Segment { data: vec![0; num_words as usize] };
        self.push_memory_space(segment);
        *self.registers.get_unchecked_mut(reg1 as usize) = (self.get_memory_space_len().wrapping_sub(1)) as u32;
    }
    
    pub unsafe fn unmapseg(&mut self, reg1: u32) {
        let reg1_value = *self.registers.get_unchecked(reg1 as usize);
        self.set_segment_from_memory_space(reg1_value, Segment { data: vec![0] });
        self.push_free_memory(reg1_value);
    }
    
    pub unsafe fn output(&mut self, reg1: u32) {
        let reg_value = *self.registers.get_unchecked(reg1 as usize) as u8;
        let output_buffer = &[reg_value];
        stdout().write_all(output_buffer).unwrap();
        stdout().flush().unwrap();
    }
    
    pub unsafe fn input(&mut self, reg1: u32) {
        let mut input_buffer = [0_u8; 1]; // Buffer to store the input byte
        if let Ok(n) = stdin().read(&mut input_buffer) {
            if n > 0 { // If a byte was successfully read
                let input_value = input_buffer[0] as u32;
                *self.registers.get_unchecked_mut(reg1 as usize) = input_value;
            }
        } 
    }
    
    pub unsafe fn loadprog(&mut self, reg1: u32, reg2: u32) {
        let reg1_value = *self.registers.get_unchecked(reg1 as usize);
        if reg1_value == 0 {
            self.set_program_counter(*self.registers.get_unchecked(reg2 as usize));
        } else {
            let segment = self.get_segment_from_memory_space(reg1_value);
            self.set_segment_from_memory_space(0, segment);
            self.set_program_counter(*self.registers.get_unchecked(reg2 as usize));
        }
    }
    
    pub unsafe fn loadval(&mut self, reg1: u32, value: u32) {
        *self.registers.get_unchecked_mut(reg1 as usize) = value;
    }
}
