use crate::rumdis;

/// A Universal Machine capable of executing programs stored in memory segments.
///
/// The UniversalMachine contains registers, a program counter, memory space, and free memory.
/// It operates by executing instructions stored in memory segments.
pub struct UniversalMachine {
    pub registers: Vec<u32>,
    pub program_counter: u32,
    pub memory_space: Vec<Vec<u32>>,
    pub free_memory: Vec<u32>,
} 

impl UniversalMachine {
    /// Creates a new instance of the UniversalMachine.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A vector containing instructions to be loaded into the memory space.
    ///
    /// # Returns
    ///
    /// A new instance of UniversalMachine.
    pub unsafe fn new(instructions: Vec<u32>) -> Self {
        let registers = vec![0; 8];  // Initialize registers with 8 elements, all set to 0
        let memory_space = vec![instructions];
        let program_counter = 0;
        let free_memory = Vec::new();  // Initialize free memory as empty vector

        UniversalMachine {
            registers,
            program_counter,
            memory_space,
            free_memory,
        }
    }

    /// Sets the program counter to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the program counter to.
    pub unsafe fn set_program_counter(&mut self, value: u32) {
        self.program_counter = value;
    }

        /// Retrieves the segment from the memory space at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the segment to retrieve.
    ///
    /// # Returns
    ///
    /// The segment at the specified address.
    pub unsafe fn get_segment_from_memory_space(&self, address: u32) -> &Vec<u32> {
        &self.memory_space[address as usize]
    }

    /// Sets the segment in the memory space at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to set the segment to.
    /// * `value` - The segment to set.
    pub unsafe fn set_segment_from_memory_space(&mut self, address: u32, value: Vec<u32>) {
        if (address as usize) < self.memory_space.len(){
            self.memory_space[address as usize] = value;
        }
    }

    /// Retrieves the value from the memory space at the specified address and offset.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the segment in the memory space.
    /// * `offset` - The offset within the segment.
    ///
    /// # Returns
    ///
    /// The value at the specified address and offset.
    pub unsafe fn get_val_from_memory_space(&self, address: u32, offset: u32) -> u32 {
        let segment = self.memory_space.get_unchecked(address as usize);
        *segment.get_unchecked(offset as usize)
    }

    /// Sets the value in the memory space at the specified address and offset.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the segment in the memory space.
    /// * `offset` - The offset within the segment.
    /// * `value` - The value to set.
    pub unsafe fn set_val_from_memory_space(&mut self, address: u32, offset: u32, value: u32) {
        let segment = self.memory_space.get_unchecked_mut(address as usize);
        *segment.get_unchecked_mut(offset as usize) = value;
    }

    /// Retrieves the length of the free memory.
    pub unsafe fn get_free_memory_len(&self) -> usize {
        self.free_memory.len()
    }

    /// Retrieves the value from the free memory at the top of the stack.
    ///
    /// # Returns
    ///
    /// The value from the top of the free memory stack.
    pub unsafe fn get_from_free_memory(&self) -> u32 {
        self.free_memory[(self.free_memory.len() - 1) as usize]
    }

    /// Runs the Universal Machine, continuously executing instructions until halted.
    pub unsafe fn run(&mut self) {
        loop {
            let instruction = self.get_val_from_memory_space(0, self.program_counter);
            rumdis::disassemble(instruction, self);
        }
    }
}
