use rayon::prelude::*;
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

    part1(state.to_owned());
    part2(state);
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
                .bitxor(literal_operand_value(&state).get() as u64)
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

fn part2(start_state: ComputerState) {
    let expected_output = start_state.operations.to_owned();

    let solution = (2u64.pow(45)..2u64.pow(48) - 1)
        .into_par_iter()
        .filter_map(|reg_a| {
            // Create new state to test in this iteration
            let mut state = ComputerState {
                reg_a,
                ..start_state.to_owned()
            };

            // Create new output buffer for this iteration
            let mut output = vec![];

            while state.instruction_pointer < state.operations.len() {
                part2_compute(&mut state, &mut output);

                let max_output_index = output.len() - 1;

                // Short circuit if latest addition isn't same as expected value
                if expected_output.get(max_output_index) != output.get(max_output_index) {
                    break;
                }
            }

            if output.len() == expected_output.len() && output.last() == expected_output.last() {
                Some(reg_a)
            } else {
                None
            }
        })
        .take_any(1)
        .min();

    println!("Reached expected output at {:?}", solution);
}

fn part2_compute(state: &mut ComputerState, output_buffer: &mut Vec<U3>) {
    let instruction_index = state.instruction_pointer;
    let instruction: Instruction = state.operations[instruction_index].into();
    let mut jumped = false;

    match instruction {
        Instruction::Adv => state.reg_a = division_instruction(&state),
        Instruction::Bxl => {
            state.reg_b = state
                .reg_b
                .bitxor(literal_operand_value(&state).get() as u64)
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
}

fn division_instruction(state: &ComputerState) -> u64 {
    let numerator = state.reg_a;
    let denominator = 2_u64.pow(combo_operand_value(state) as u32);
    numerator / denominator
}

fn combo_operand_value(state: &ComputerState) -> u64 {
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
