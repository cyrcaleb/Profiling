use std::env;
use rum::rumload;
use rum::universal_machine;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut um = universal_machine::UniversalMachine::new(instructions);
    um.run();
}
