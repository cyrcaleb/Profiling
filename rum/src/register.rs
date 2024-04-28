use super::universal_machine::UniversalMachine;
use std::io::{stdin, stdout, Read, Write};
use std::mem::MaybeUninit;

impl UniversalMachine {
    pub unsafe fn cmov(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg1_value = *self.registers.get_unchecked(reg1);
        if reg1_value != 0 {
            *self.registers.get_unchecked_mut(reg2) = *self.registers.get_unchecked(reg3);
        }
    }
    
    pub unsafe fn load(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg2_value = *self.registers.get_unchecked(reg2);
        let reg3_value = *self.registers.get_unchecked(reg3);
        *self.registers.get_unchecked_mut(reg1) = *self.memory_space.get_unchecked(reg2_value as usize).get_unchecked(reg3_value as usize);
    }
    
    pub unsafe fn store(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg1_value = *self.registers.get_unchecked(reg1);
        let reg2_value = *self.registers.get_unchecked(reg2);
        let reg3_value = *self.registers.get_unchecked(reg3);
        *self.memory_space.get_unchecked_mut(reg1_value as usize).get_unchecked_mut(reg2_value as usize) = reg3_value;
    }

    pub unsafe fn add(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg2_value = *self.registers.get_unchecked(reg2);
        let reg3_value = *self.registers.get_unchecked(reg3);
        *self.registers.get_unchecked_mut(reg1) = reg2_value.wrapping_add(reg3_value);
    }
    
    pub unsafe fn mul(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg2_value = *self.registers.get_unchecked(reg2);
        let reg3_value = *self.registers.get_unchecked(reg3);
        *self.registers.get_unchecked_mut(reg1) = reg2_value.wrapping_mul(reg3_value);
    }
    
    pub unsafe fn div(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg3_value = *self.registers.get_unchecked(reg3);
        if reg3_value != 0 {
            let reg2_value = *self.registers.get_unchecked(reg2);
            *self.registers.get_unchecked_mut(reg1) = reg2_value.wrapping_div(reg3_value);
        }
    }
    
    pub unsafe fn nand(&mut self, reg1: usize, reg2: usize, reg3: usize) {
        let reg2_value = *self.registers.get_unchecked(reg2);
        let reg3_value = *self.registers.get_unchecked(reg3);
        *self.registers.get_unchecked_mut(reg1) = !(reg2_value & reg3_value);
    }
    
    pub unsafe fn halt(&mut self) {
        std::process::exit(0);
    }
    
    pub unsafe fn mapseg(&mut self, reg1: usize, reg2: usize) {
        if let Some(&last_free_mem) = self.free_memory.last() {
            let num_words = *self.registers.get_unchecked(reg2);
            let segment = vec![0; num_words as usize];
            *self.memory_space.get_unchecked_mut(last_free_mem as usize) = segment;
            *self.registers.get_unchecked_mut(reg1) = last_free_mem;
            self.free_memory.pop();
            return;
        }
        let num_words = *self.registers.get_unchecked(reg2);
        let segment = vec![0; num_words as usize];
        self.memory_space.push(segment);
        *self.registers.get_unchecked_mut(reg1) = (self.memory_space.len() - 1) as u32;
    }
    
    pub unsafe fn unmapseg(&mut self, reg1: usize) {
        let reg1_value = *self.registers.get_unchecked(reg1);
        let memory_ptr = self.memory_space
            .get_unchecked_mut(reg1_value as usize)
            .as_mut_ptr() as *mut MaybeUninit<u32>;
        std::ptr::write_volatile(memory_ptr, MaybeUninit::uninit());
        self.free_memory.push(reg1_value);
    }
    
    pub unsafe fn output(&mut self, reg1: usize) {
        let reg_value = *self.registers.get_unchecked(reg1) as u8;
        stdout().write_all(&[reg_value]).unwrap();
        stdout().flush().unwrap();
    }
    
    pub unsafe fn input(&mut self, reg1: usize) {
        let mut input_buffer = [0_u8; 1];
        if let Ok(n) = stdin().read(&mut input_buffer) {
            if n > 0 {
                *self.registers.get_unchecked_mut(reg1) = input_buffer[0] as u32;
            }
        } 
    }
    
    pub unsafe fn loadprog(&mut self, reg1: usize, reg2: usize) {
        let reg1_value = *self.registers.get_unchecked(reg1);
        if reg1_value == 0 {
            self.program_counter = *self.registers.get_unchecked(reg2);
        } else {
            let segment = self.memory_space.swap_remove(reg1_value as usize);
            self.memory_space.insert(0, segment);
            self.program_counter = *self.registers.get_unchecked(reg2);
        }
    }
    
    
    pub unsafe fn loadval(&mut self, reg1: usize, value: usize) {
        *self.registers.get_unchecked_mut(reg1) = value as u32;
    }
}
