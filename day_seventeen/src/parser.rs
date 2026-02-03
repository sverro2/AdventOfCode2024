use winnow::Result;
use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{preceded, separated, terminated};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::literal;

use crate::ComputerState;
use crate::u3::U3;

fn register<'a>(name: &'static str) -> impl Parser<&'a str, u64, ContextError> {
    preceded(
        (literal("Register "), literal(name), literal(": ")),
        dec_uint,
    )
}

fn program_ops(input: &mut &str) -> Result<Vec<U3>> {
    separated(
        1..,
        dec_uint::<_, u8, _>
            .verify(|&v| v <= 7)
            .map(|n| U3::new(n).unwrap()),
        literal(","),
    )
    .parse_next(input)
}

pub fn computer_state(input: &mut &str) -> Result<ComputerState> {
    let reg_a = terminated(register("A"), newline).parse_next(input)?;
    let reg_b = terminated(register("B"), newline).parse_next(input)?;
    let reg_c = terminated(register("C"), (newline, newline)).parse_next(input)?;

    let operations = preceded(literal("Program: "), program_ops).parse_next(input)?;

    Ok(ComputerState {
        reg_a,
        reg_b,
        reg_c,
        instruction_pointer: 0,
        operations,
    })
}
