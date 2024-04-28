use std::env;
use rum::rumload;
use rum::universal_machine;

fn main() {
    let input = env::args().nth(1);
    let instructions = unsafe { rumload::load(input.as_deref()) }; 
    let mut um = unsafe { universal_machine::UniversalMachine::new(instructions) }; 
    unsafe { um.run() }; 
}
