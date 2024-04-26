use crate::rumdis;

/// A Universal Machine capable of executing programs stored in memory segments.
///
/// The UniversalMachine contains registers, a program counter, memory space, and free memory.
/// It operates by executing instructions stored in memory segments.
pub struct UniversalMachine {
    registers: Vec<u32>,
    program_counter: u32,
    memory_space: Vec<Segment>,
    free_memory: Vec<u32>,
} 

/// Represents a memory segment containing data.
///
/// Segments are used by the UniversalMachine to store program instructions or data.
#[derive(Clone)]
pub struct Segment {
    pub data: Vec<u32>,
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
    pub fn new(instructions: Vec<u32>) -> Self {
        let registers = vec![0; 8];  // Initialize registers with 8 elements, all set to 0
        let memory_space = vec![Segment{data: instructions}];
        let program_counter = 0;
        let free_memory = Vec::new();  // Initialize free memory as empty vector

        UniversalMachine {
            registers,
            program_counter,
            memory_space,
            free_memory,
        }
    }

    /// Increments the program counter by 1.
    pub fn increment_counter(&mut self) {
        self.program_counter += 1;
    }

    /// Sets the program counter to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the program counter to.
    pub fn set_program_counter(&mut self, value: u32) {
        self.program_counter = value;
    }

    /// Retrieves the value of the register at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the register to retrieve.
    ///
    /// # Returns
    ///
    /// The value of the register.
    pub fn get_register(&self, index: usize) -> u32 {
        self.registers[index]
    }

    /// Sets the value of the register at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the register to set.
    /// * `value` - The value to set the register to.
    pub fn set_register(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
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
    pub fn get_segment_from_memory_space(&self, address: u32) -> Segment {
        return self.memory_space[address as usize].clone();
    }

    /// Sets the segment in the memory space at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to set the segment to.
    /// * `value` - The segment to set.
    pub fn set_segment_from_memory_space(&mut self, address: u32, value: Segment) {
        if (address as usize) < self.memory_space.len(){
            self.memory_space[address as usize] = value;
        } else{
            eprintln!("Error: Address space out of range.");
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
    pub fn get_val_from_memory_space(&self, address: u32, offset: u32) -> u32 {
        return self.memory_space[address as usize].data[offset as usize];
    }

    /// Sets the value in the memory space at the specified address and offset.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the segment in the memory space.
    /// * `offset` - The offset within the segment.
    /// * `value` - The value to set.
    pub fn set_val_from_memory_space(&mut self, address: u32, offset: u32, value: u32) {
        self.memory_space[address as usize].data[offset as usize] = value;
    }
    
    /// Retrieves the length of the free memory.
    pub fn get_free_memory_len(&self) -> usize {
        return self.free_memory.len();
    }

    /// Retrieves the length of the memory space.
    pub fn get_memory_space_len(&self) -> usize {
        return self.memory_space.len();
    }

    /// Retrieves the value from the free memory at the top of the stack.
    ///
    /// # Returns
    ///
    /// The value from the top of the free memory stack.
    pub fn get_from_free_memory(&self) -> u32 {
        return self.free_memory[(self.free_memory.len() - 1) as usize];
    }

    /// Pushes a segment onto the memory space.
    ///
    /// # Arguments
    ///
    /// * `value` - The segment to push onto the memory space.
    pub fn push_memory_space(&mut self, value: Segment) {
        self.memory_space.push(value);
    }

    /// Pushes an address onto the free memory stack.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to push onto the free memory stack.
    pub fn push_free_memory(&mut self, address: u32) {
        self.free_memory.push(address);
    }

    /// Pops an address from the free memory stack.
    ///
    /// # Returns
    ///
    /// The address popped from the free memory stack.
    pub fn pop_free_memory(&mut self) -> u32 {
        self.free_memory.pop().unwrap()
    }

    /// Peeks at the top address from the free memory stack without removing it.
    ///
    /// # Returns
    ///
    /// An option containing the top address from the free memory stack, or None if the stack is empty.
    pub fn peek_free_memory(&self) -> Option<u32> {
        if self.free_memory.is_empty() {
            return None;
        } else {
            return Some((self.free_memory.len()-1) as u32);
        }
    }

    /// Runs the Universal Machine, continuously executing instructions until halted.
    pub fn run(&mut self) {
        loop {
            let instruction = self.get_val_from_memory_space(0, self.program_counter);
            rumdis::disassemble(instruction, self);
        }
    }
} 
