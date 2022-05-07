use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("../inputs/e1_sonar_sweeper_input.txt").expect("Cannot read the file");

    let mut measurements = Vec::new();

    contents.lines().for_each(|line| {
        measurements.push(line.parse::<u32>().unwrap());
    });

    println!("--- Part One ---");
    println!("There are {} measurements greater than previous measurements", count_increases(&measurements));

    println!("--- Part Two ---");
    println!("There are {} measurements sums greater than previous measurements sums", count_increases(&group_measurements_by_windows(&measurements)));
}

fn count_increases(measurements: &Vec<u32>) -> u32 {
    let mut count = 0;

    measurements.iter().enumerate().for_each(|(i, _)| {
        if i != 0 {
            if measurement_has_increased(measurements[i - 1], measurements[i]) {
                count += 1;
            }
        }
    });
    return count
}


fn group_measurements_by_windows(measurements: &Vec<u32>) -> Vec<u32> {
    let mut windows = Vec::new();

    // measurements.len() - 2 skips the last two elements
    for i in 0..measurements.len() - 2 {
        windows.push(measurements[i..i + 3].iter().sum())
    }

    return windows
}

fn measurement_has_increased(last_measurement: u32, current_measurement: u32) -> bool {
    return current_measurement > last_measurement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_the_measurement_has_increased_it_can_be_detected() {
        assert_eq!(measurement_has_increased(0, 1), true)
    }

    #[test]
    fn if_the_measurement_has_not_increased_it_can_be_detected() {
        assert_eq!(measurement_has_increased(0, 0), false)
    }

    #[test]
    fn measurements_are_correctly_grouped_by_window() {
        let measurements = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(group_measurements_by_windows(&measurements), &[6, 9, 12, 15])
    }

    #[test]
    fn count_number_of_increases() {
        let measurements = vec![1, 2, 2, 3, 3, 4];
        assert_eq!(count_increases(&measurements), 3)
    }
}