use std::cmp::max;
use std::collections::HashMap;

fn neighbours(row: isize, col: isize) -> Vec<(isize, isize)> {
    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

pub fn starting_state(input: &str) -> (Vec::<(isize, isize)>,  HashMap::<(isize, isize), u8>){
    let mut adjacent_rolls = HashMap::<(isize, isize), u8>::new();
    let mut roll_positions = Vec::<(isize, isize)>::new();



    input.lines().enumerate().for_each(|(r, row)| {
        row.chars().enumerate().for_each(|(c, char)| {
            if char != '@' { return; }

            roll_positions.push((r as isize, c as isize));
            for (x, y) in neighbours(r as isize, c as isize) {
                *adjacent_rolls.entry((x, y)).or_insert(0) += 1;
            }
        })
    });


    (roll_positions, adjacent_rolls)
}

#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    let (mut roll_positions, mut adjacent_rolls) = starting_state(input);

    roll_positions.iter()
        .filter(|pos| {
            *adjacent_rolls.get(*pos).unwrap_or(&0) < 4
        }).count() as u64
    // })
    //     .for_each(|(r, c)| {
    //    println!("{}", format!("{r},{c} : {}", adjacent_rolls.get(&(*r,*c) ).unwrap_or(&0) ));
    // });
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let (mut roll_positions, mut adjacent_rolls) = starting_state(input);
    let mut total_rolls_removed: u64 = 0;

    let mut rolls_removed_current_iteration = 1;
    while rolls_removed_current_iteration != 0 {
        rolls_removed_current_iteration = 0;
        let mut new_roll_positions = roll_positions.clone();

        roll_positions.iter().enumerate().for_each(|(idx, (r, c))| {
            if (*adjacent_rolls.get(&(*r,*c)).unwrap_or(&0) >= 4){
                return; // Ignore this for the next run
            }

            // Remove it from the roll positions
            new_roll_positions.remove(idx - rolls_removed_current_iteration);
            // Subtract 1 from all previous issues
            for (x, y) in neighbours(*r, *c) {
                *adjacent_rolls.entry((x, y)).or_insert(0) -= 1;
            }

            rolls_removed_current_iteration += 1;
        });

        total_rolls_removed += rolls_removed_current_iteration as u64;
        roll_positions = new_roll_positions.clone();
    }

    total_rolls_removed
}
