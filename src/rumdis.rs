use num_derive::FromPrimitive; 
use num_traits::FromPrimitive; type Umi = u32;
use crate::universal_machine::UniversalMachine;

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)] #[repr(u32)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSeg,
    UnmapSeg,
    Out,
    In,
    LoadProg,
    LoadVal,
}

pub struct Field { 
    width: u32,
    lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3}; 
static RC: Field = Field {width: 3, lsb: 0}; 
static RL: Field = Field {width: 3, lsb: 25}; 
static VL: Field = Field {width: 25, lsb: 0}; 
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

fn op(instruction: Umi) -> Option<Opcode> { 
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn disassemble(inst: Umi, um: &mut UniversalMachine) {
    // um.print_state();
    um.increment_counter();
    match op(inst) {
        Some(Opcode::CMov) => {
            um.cmov(get(&RC, inst), get(&RA, inst), get(&RB, inst));
        }
        Some(Opcode::Load) => {
            um.load(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Store) => {
            um.store(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Add) => {
            um.add(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Mul) => {
            um.mul(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Div) => {
            um.div(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Nand) => {
            um.nand(get(&RA, inst), get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::Halt) => {
            um.halt();
        }
        Some(Opcode::MapSeg) => {
            um.mapseg(get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::UnmapSeg) => {
            um.unmapseg(get(&RC, inst));
        }
        Some(Opcode::Out) => {
            um.output(get(&RC, inst));
        }
        Some(Opcode::In) => {
            um.input(get(&RC, inst));
        }
        Some(Opcode::LoadProg) => {
            um.loadprog(get(&RB, inst), get(&RC, inst));
        }
        Some(Opcode::LoadVal) => {
            um.loadval(get(&RL, inst),get(&VL, inst));
        }
        None => {
            panic!("Invalid opcode");
        }
    }
}
