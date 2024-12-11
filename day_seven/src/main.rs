use winnow::ascii::digit1;
use winnow::ascii::newline;
use winnow::ascii::space1;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::combinator::terminated;
use winnow::PResult;
use winnow::Parser;

#[derive(Debug)]
struct Equation {
    answer: u64,
    parts: Vec<u16>,
}

#[derive(Clone)]
enum Operator {
    Sum,
    Multiply,
    Concat,
}

impl Operator {
    fn next(&self) -> Option<Self> {
        match self {
            Operator::Sum => Some(Operator::Multiply),
            Operator::Multiply => Some(Operator::Concat),
            Operator::Concat => None,
        }
    }

    fn start() -> Self {
        Self::Sum
    }
}

struct OperatorList {
    operators: Vec<Operator>
}

impl OperatorList {
    fn at(&self, index :usize) -> Operator {
        if index < self.operators.len() {
            todo!()
        } else {
            Operator::start()
        }
    }

    fn new() -> OperatorList {
        Self {operators: vec![Operator::start()]}
    }

    fn next(&mut self) {

    }
}

fn main() {
    let input = parse_equations();
    part_1(&input);
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

fn check_is_solvable_v2(equation: &Equation) -> bool {
    let amount_of_operants = equation.parts.len() as u32 - 1;

    // Now there are three options. Things get more complex: '+', '*' and 'concat'
    // I need more data, so now twice the bits. Could even add another operant in the future.
    let possible_variations = u16::pow(4, amount_of_operants);

    (0..possible_variations).any(|operant_configuration| {
        let mut accumulated = *equation
            .parts
            .first()
            .expect("every equation needs at least one part") as u64;

        for (index, next_item) in equation.parts.iter().skip(1).enumerate() {
            if accumulated > equation.answer {
                break;
            }

            let is_odd_index = index % 2 == 1;

            let operator = if is_odd_index {
                if operant_configuration >> index & 1 == 1;
            } else {

            }

            if use_multiply {
                accumulated *= (*next_item) as u64
            } else {
                accumulated += (*next_item) as u64
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

fn parse_equation(input: &mut &str) -> PResult<Equation> {
    let answer: u64 = terminated(digit1, (':', space1))
        .try_map(str::parse)
        .parse_next(input)?;

    let parts: Vec<u16> = parse_equation_parts(input)?;

    Ok(Equation { answer, parts })
}

fn parse_equation_parts(input: &mut &str) -> PResult<Vec<u16>> {
    terminated(
        separated(1.., digit1.try_map(str::parse::<u16>), space1),
        newline,
    )
    .parse_next(input)
}
