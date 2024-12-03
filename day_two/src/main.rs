fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");

    let safe_count_exercise_1 = excercise_1(input);
    let safe_count_exercise_2 = excercise_2(input);

    println!("Safe count in excersize 1: {}", safe_count_exercise_1);
    println!("Safe count in excersize 2: {}", safe_count_exercise_2);

    println!("It took {:?}", start.elapsed());
}

#[derive(PartialEq)]
enum ReportType {
    Increasing,
    Decreasing,
}

fn excercise_1(input: &str) -> usize {
    let amount_of_safe_reports = input
        .lines()
        .filter(|line| {
            let levels_in_line: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();

            let mut type_of_report = None;
            let found_any_unsafe = levels_in_line.windows(2).any(|slice| {
                let latest_delta_type = if slice[0] < slice[1] {
                    Some(ReportType::Increasing)
                } else {
                    Some(ReportType::Decreasing)
                };

                // If type of report is known, it can not change anymore
                if type_of_report.is_some() {
                    if type_of_report != latest_delta_type {
                        return true;
                    }
                } else {
                    type_of_report = latest_delta_type;
                }

                // Also check if delta is within acceptable margins
                let delta_value = (slice[0] - slice[1]).abs();
                if !(1..=3).contains(&delta_value) {
                    return true;
                }

                false
            });

            !found_any_unsafe
        })
        .count();

    amount_of_safe_reports
}

fn excercise_2(input: &str) -> usize {
    let amount_of_safe_inputs = input
        .lines()
        .filter(|line| {
            let numbers_in_list: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();

            let index_with_error = safety_of_report(&numbers_in_list);

            if let Some(index) = index_with_error {
                // Problem can be found with reported index or the index after it
                let mut left_value_removed_list = numbers_in_list;
                let mut right_value_removed_list = left_value_removed_list.clone();
                left_value_removed_list.remove(index);
                right_value_removed_list.remove(index + 1);

                safety_of_report(&left_value_removed_list).is_none()
                    || safety_of_report(&right_value_removed_list).is_none()
            } else {
                true
            }
        })
        .count();

    amount_of_safe_inputs
}

fn safety_of_report(numbers_in_line: &[i32]) -> Option<usize> {
    let mut type_of_report = None;
    let found_any_unsafe = numbers_in_line.windows(2).enumerate().find(|(_, slice)| {
        let latest_delta_kind = if slice[0] < slice[1] {
            Some(ReportType::Increasing)
        } else {
            Some(ReportType::Decreasing)
        };

        // If type of report is known, it can not change anymore
        if type_of_report.is_some() {
            if type_of_report != latest_delta_kind {
                return true;
            }
        } else {
            type_of_report = latest_delta_kind;
        }

        // Also check if delta is within acceptable margins
        let delta_value = (slice[0] - slice[1]).abs();
        if !(1..=3).contains(&delta_value) {
            return true;
        }

        false
    });

    found_any_unsafe.map(|(index, _)| index)
}
