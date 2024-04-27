use std::env;
use profiling::rumload;
use profiling::universal_machine;

fn main() {
    let input = env::args().nth(1);
    let instructions = unsafe { rumload::load(input.as_deref()) }; // Unsafe loading of instructions
    let mut um = unsafe { universal_machine::UniversalMachine::new(instructions) }; // Unsafe creation of UniversalMachine
    unsafe { um.run() }; // Unsafe execution of the UniversalMachine
}
