use crate::u3::U3;

#[derive(Debug)]
pub struct ComputerState {
    pub reg_a: u32,
    pub reg_b: u32,
    pub reg_c: u32,
    pub instruction_pointer: usize,
    pub operations: Vec<U3>,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<U3> for Instruction {
    fn from(value: U3) -> Self {
        // We know casting can be safely done
        unsafe { std::mem::transmute::<u8, Instruction>(value.get()) }
    }
}

#[repr(u8)]
pub enum ComboOperand {
    Literal0,
    Literal1,
    Literal2,
    Literal3,
    ValueOfA,
    ValueOfB,
    ValueOfC,
    Reserved,
}

impl From<U3> for ComboOperand {
    fn from(value: U3) -> Self {
        // We know casting can be safely done
        unsafe { std::mem::transmute::<u8, ComboOperand>(value.get()) }
    }
}
