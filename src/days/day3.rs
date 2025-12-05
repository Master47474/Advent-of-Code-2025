use std::cmp::max;

#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    input.lines()
        .map(|line|
        {
            let batteries = line.as_bytes();
            max_joltage(batteries, 2)
        })
        .sum()
}


#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    input.lines()
        .map(|line|
            {
                let batteries = line.as_bytes();
                max_joltage(batteries, 12)
            })
        .sum()
}

fn max_joltage(batteries: &[u8], max_batteries: usize) -> u64 {
    let mut digits = vec![0; max_batteries];
    digits[0] = u64::from(batteries[0] - b'0');

    for i in 1..batteries.len() {
        let digit_to_insert = u64::from(batteries[i] - b'0');

        let mut reset_remaining_digits_flag = false;
        // This is negative until we are in the last max_battery digits

        let potential_start = i as isize - (batteries.len() - max_batteries) as isize;
        let compare_window_start_offset = max(0, potential_start) as usize;

        for d in digits.iter_mut().skip(compare_window_start_offset) {
            if reset_remaining_digits_flag {
                *d = 0;
            }
            else if digit_to_insert > *d {
                *d = digit_to_insert;
                reset_remaining_digits_flag = true;
            }
        }
    }

    digits.iter().fold(0, |acc, digit| (acc*10) + *digit)
}