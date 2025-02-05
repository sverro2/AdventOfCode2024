use std::time::Instant;

use operator::Operator;
use operator::OperatorList;
use rayon::prelude::*;
use winnow::ascii::digit1;
use winnow::ascii::newline;
use winnow::ascii::space1;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::combinator::terminated;
use winnow::Parser;
use winnow::Result;

mod operator;

#[derive(Debug)]
struct Equation {
    answer: u64,
    parts: Vec<u16>,
}

fn main() {
    let input = parse_equations();
    part_1(&input);

    let start = Instant::now();
    part_2(&input);
    println!("Time taken: {:?}", start.elapsed());
}

fn part_1(equations: &[Equation]) {
    let solvable_equation_sum: u64 = equations
        .iter()
        .filter_map(|e| check_is_solvable(e).then_some(e.answer))
        .sum();

    println!(
        "Total sum of solvable equation answers: {}",
        solvable_equation_sum
    );
}

fn part_2(equations: &[Equation]) {
    let solvable_equation_sum: u64 = equations
        .par_iter()
        .filter_map(|e| check_is_solvable_part2(e).then_some(e.answer))
        .sum();

    println!(
        "Total sum of solvable equation answers: {}",
        solvable_equation_sum
    );
}

fn check_is_solvable(equation: &Equation) -> bool {
    let amount_of_operants = equation.parts.len() as u32 - 1;

    // In binary there are two options. In this case there are two options as well: '+' and '*'
    let possible_variations = u16::pow(2, amount_of_operants);

    (0..possible_variations).any(|operant_configuration| {
        let mut accumulated = *equation
            .parts
            .first()
            .expect("every equation needs at least one part") as u64;

        for (index, next_item) in equation.parts.iter().skip(1).enumerate() {
            if accumulated > equation.answer {
                break;
            }

            let use_multiply = operant_configuration >> index & 1 == 1;

            if use_multiply {
                accumulated *= (*next_item) as u64
            } else {
                accumulated += (*next_item) as u64
            }
        }

        accumulated == equation.answer
    })
}

fn check_is_solvable_part2(equation: &Equation) -> bool {
    let amount_of_operants = equation.parts.len() as u32 - 1;

    let possible_variations = usize::pow(3, amount_of_operants);
    let operator_list = OperatorList::new();

    operator_list
        .take(possible_variations)
        .any(|operant_configuration| {
            let mut accumulated = *equation
                .parts
                .first()
                .expect("every equation needs at least one part")
                as u64;

            for (index, next_item) in equation.parts.iter().skip(1).enumerate() {
                if accumulated > equation.answer {
                    break;
                }

                match operant_configuration.at(index) {
                    Operator::Sum => accumulated += *next_item as u64,
                    Operator::Multiply => accumulated *= *next_item as u64,
                    Operator::Concat => {
                        accumulated = format!("{accumulated}{next_item}").parse().unwrap()
                    }
                }
            }

            accumulated == equation.answer
        })
}

fn parse_equations() -> Vec<Equation> {
    let mut input = include_str!("../input.txt");
    repeat(1.., parse_equation)
        .parse_next(&mut input)
        .expect("unable to parse input file")
}

fn parse_equation(input: &mut &str) -> Result<Equation> {
    let answer: u64 = terminated(digit1, (':', space1))
        .try_map(str::parse)
        .parse_next(input)?;

    let parts: Vec<u16> = parse_equation_parts(input)?;

    Ok(Equation { answer, parts })
}

fn parse_equation_parts(input: &mut &str) -> Result<Vec<u16>> {
    terminated(
        separated(1.., digit1.try_map(str::parse::<u16>), space1),
        newline,
    )
    .parse_next(input)
}
