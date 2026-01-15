use std::ops::BitXor;

use crate::{
    types::{ComboOperand, ComputerState, Instruction},
    u3::U3,
};

mod parser;
mod types;
mod u3;

fn main() {
    let mut input = include_str!("../input.txt");
    let state = parser::computer_state(&mut input).unwrap();

    part1(state);
}

fn part1(start_state: ComputerState) {
    let mut output = vec![];

    part1_compute(start_state, &mut output);
    println!("Output {}", output.join(","));
}

fn part1_compute(mut state: ComputerState, output_buffer: &mut Vec<U3>) {
    let instruction_index = state.instruction_pointer;
    let instruction: Instruction = state.operations[instruction_index].into();
    let mut jumped = false;

    match instruction {
        Instruction::Adv => state.reg_a = division_instruction(&state),
        Instruction::Bxl => {
            state.reg_b = state
                .reg_b
                .bitxor(literal_operand_value(&state).get() as u32)
        }
        Instruction::Bst => state.reg_b = combo_operand_value(&state) % 8,
        Instruction::Jnz => {
            if state.reg_a != 0 {
                jumped = true;
                state.instruction_pointer = literal_operand_value(&state).get() as usize;
            }
        }
        Instruction::Bxc => state.reg_b = state.reg_b.bitxor(state.reg_c),
        Instruction::Out => output_buffer.push(U3::new(combo_operand_value(&state) % 8).unwrap()),
        Instruction::Bdv => state.reg_b = division_instruction(&state),
        Instruction::Cdv => state.reg_c = division_instruction(&state),
    }

    if !jumped {
        state.instruction_pointer += 2;
    }

    if state.instruction_pointer < state.operations.len() {
        // println!("{state:?}");
        part1_compute(state, output_buffer);
    }
}

fn division_instruction(state: &ComputerState) -> u32 {
    let numerator = state.reg_a;
    let denominator = 2_u32.pow(combo_operand_value(state));
    numerator / denominator
}

fn combo_operand_value(state: &ComputerState) -> u32 {
    let operant_index = state.instruction_pointer + 1;
    let operand: ComboOperand = state.operations[operant_index].into();

    match operand {
        ComboOperand::Literal0 => 0,
        ComboOperand::Literal1 => 1,
        ComboOperand::Literal2 => 2,
        ComboOperand::Literal3 => 3,
        ComboOperand::ValueOfA => state.reg_a,
        ComboOperand::ValueOfB => state.reg_b,
        ComboOperand::ValueOfC => state.reg_c,
        ComboOperand::Reserved => panic!("Program used a reserved value as operand. Aborting."),
    }
}

fn literal_operand_value(state: &ComputerState) -> U3 {
    let operant_index = state.instruction_pointer + 1;
    state.operations[operant_index]
}
