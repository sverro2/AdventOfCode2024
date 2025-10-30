use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct MultiplierCall {
    x: i32,
    y: i32,
}

impl MultiplierCall {
    fn execute(&self) -> i32 {
        self.x * self.y
    }
}

fn main() {
    let time_at_start = std::time::Instant::now();
    exercise_1();
    exercise_2();
    println!("Total time: {:?}", time_at_start.elapsed());
}

fn exercise_1() {
    let input = include_str!("../input.txt");

    let valid_multiplier_calls = Regex::new(
        r"(?x)
        mul
            \(
                (\d{1,3}) # caputure x
            ,
                (\d{1,3}) # caputure y
            \)
        ",
    )
    .unwrap();

    let all_multiplecations_sum: i32 = valid_multiplier_calls
        .captures_iter(input)
        .map(|capt| {
            let (_, [x, y]) = capt.extract();

            let call = MultiplierCall {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };

            call.execute()
        })
        .sum();
    println!("{all_multiplecations_sum}");
}

fn exercise_2() {
    let input = include_str!("../input.txt");

    let valid_multiplier_calls = Regex::new(
        r"(?x)
        mul
            \(
                (\d{1,3}) # caputure x
            ,
                (\d{1,3}) # caputure y
            \)
        ",
    )
    .unwrap();

    // Depending where an multiplier call is located, we want to toggle the multiplier on or off.
    let toggles = MultiplierToggles::from_str(input).unwrap();

    let all_multiplications_sum: i32 = valid_multiplier_calls
        .captures_iter(input)
        .filter_map(|captured| {
            let index_of_capture = captured.get(0).map(|i| i.start()).unwrap();
            let (_, [x, y]) = captured.extract();

            let call = MultiplierCall {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };

            let toggle_state = toggles.check_state_at_index(index_of_capture);

            if matches!(toggle_state, MultiplierToggleState::On) {
                Some(call.execute())
            } else {
                None
            }
        })
        .sum();
    println!("{all_multiplications_sum}");
}

#[derive(Debug)]
struct MultiplierToggles {
    toggles: Vec<MultiplierToggle>,
}

impl MultiplierToggles {
    fn check_state_at_index(&self, index: usize) -> MultiplierToggleState {
        let mut last_toggle = None;
        for toggle in &self.toggles {
            if toggle.index >= index {
                break;
            } else {
                last_toggle = Some(toggle);
            }
        }

        // return last toggle value. Even if not toggles have not been found yet, we start with 'On' state.
        last_toggle
            .map(|t| t.state.clone())
            .unwrap_or(MultiplierToggleState::On)
    }
}

impl FromStr for MultiplierToggles {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let switch_on_indices: Vec<_> = s
            .match_indices("do()")
            .map(|(index, _)| MultiplierToggle {
                index,
                state: MultiplierToggleState::On,
            })
            .collect();
        let switch_off_toggles: Vec<_> = s
            .match_indices("don't()")
            .map(|(index, _)| MultiplierToggle {
                index,
                state: MultiplierToggleState::Off,
            })
            .collect();

        let mut all_toggles = switch_on_indices;
        all_toggles.extend(switch_off_toggles);
        all_toggles.sort_by_key(|t| t.index);

        Ok(MultiplierToggles {
            toggles: all_toggles,
        })
    }
}

#[derive(Debug)]
struct MultiplierToggle {
    index: usize,
    state: MultiplierToggleState,
}

#[derive(Debug, Clone)]
enum MultiplierToggleState {
    On,
    Off,
}
