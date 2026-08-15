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
    let operation_count = start_state.operations.len();
    let solution = lowest_quine_seed(&start_state, 0, operation_count);

    println!("Reached expected output at {:?}", solution);
}

/// One loop of the program consumes the lowest 3 bits of A and emits a single
/// value, so the last emitted value only depends on the highest 3 bits of A.
/// That lets us fix A three bits at a time, working from the back of the program
/// towards the front, and only keeping the prefixes whose run already reproduces
/// the tail of the program. A prefix can still turn out to be a dead end later
/// (`cdv B` peeks at bits above the current window), hence the backtracking.
///
/// `reg_a` is the value fixed so far, `tail_start` the index in `operations` from
/// which the program still has to be reproduced.
fn lowest_quine_seed(start_state: &ComputerState, reg_a: u64, tail_start: usize) -> Option<u64> {
    if tail_start == 0 {
        return Some(reg_a);
    }

    let expected_output = &start_state.operations[tail_start - 1..];

    // Ascending, so the first solution we complete is also the lowest one
    for next_bits in 0..8 {
        let candidate = reg_a * 8 + next_bits;

        // A of 0 halts immediately, it can never be part of a solution
        if candidate == 0 {
            continue;
        }

        if run_program(start_state, candidate) == expected_output
            && let Some(solution) = lowest_quine_seed(start_state, candidate, tail_start - 1)
        {
            return Some(solution);
        }
    }

    None
}

fn run_program(start_state: &ComputerState, reg_a: u64) -> Vec<U3> {
    let mut state = ComputerState {
        reg_a,
        instruction_pointer: 0,
        ..start_state.to_owned()
    };
    let mut output = vec![];

    while state.instruction_pointer < state.operations.len() {
        part2_compute(&mut state, &mut output);
    }

    output
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
