fn main() {
    let input = include_str!("../input.txt");

    let input = input
        .lines()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let possible_headings = vec![
        Heading { x: 0, y: -1 },
        Heading { x: 1, y: -1 },
        Heading { x: 1, y: 0 },
        Heading { x: 1, y: 1 },
        Heading { x: 0, y: 1 },
        Heading { x: -1, y: 1 },
        Heading { x: -1, y: 0 },
        Heading { x: -1, y: -1 },
    ];

    let mut total_xmas_count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let total_xmas_count_for_index: usize =
                possible_headings.iter().fold(0, |acc, heading| {
                    let mut input_character_iterator = TwoDimensionalIterator {
                        current_index: Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        heading,
                        input: &input,
                    };

                    static XMAS: &str = "XMAS";

                    let failed_match = XMAS.chars().any(|character| {
                        input_character_iterator
                            .next()
                            .map(|i| i != character)
                            .unwrap_or(true)
                    });

                    if failed_match {
                        acc
                    } else {
                        acc + 1
                    }
                });

            total_xmas_count = total_xmas_count + total_xmas_count_for_index;
        }
    }

    println!("Total XMAS count: {}", total_xmas_count);
}

struct Point {
    x: i32,
    y: i32,
}

struct Heading {
    x: i32,
    y: i32,
}

struct TwoDimensionalIterator<'a> {
    current_index: Point,
    heading: &'a Heading,
    input: &'a Vec<Vec<char>>,
}

impl<'a> Iterator for TwoDimensionalIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index.y < 0
            || self.current_index.x < 0
            || self.current_index.y >= self.input.len() as i32
            || self.current_index.x >= self.input[self.current_index.y as usize].len() as i32
        {
            return None;
        }

        let item = &self.input[self.current_index.y as usize][self.current_index.x as usize];

        self.current_index = Point {
            x: self.current_index.x + self.heading.x,
            y: self.current_index.y + self.heading.y,
        };

        Some(*item)
    }
}
