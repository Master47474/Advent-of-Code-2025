
#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    let mut line_iter = input.lines();
    let mut splits = 0;
    if let Some(first) = line_iter.next() {
        // Lets set up the mask
        let mut mask: Vec<u8> = first.chars().map(|c| if c == 'S' { 1 } else { 0 }).collect();

        // Rest of the lines
        for line in line_iter {
            if !line.contains('^'){
                continue;
            }

            line.chars().enumerate().for_each(|(i, c)| {
                if !(c == '^' &&  mask[i] == 1) {
                    return;
                }

                if i > 0 {
                    mask[i-1] = 1;
                }

                mask[i] = 0;
                splits += 1;


                if i < mask.len()-1 {
                    mask[i+1] = 1;
                }
            });
        }
    }

    splits
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let mut line_iter = input.lines();
    if let Some(first) = line_iter.next() {
        // Lets set up the mask
        let mut timelines: Vec<u64> = first.chars().map(|c| if c == 'S' { 1 } else { 0 }).collect();

        // Rest of the lines
        for line in line_iter {
            if !line.contains('^'){
                continue;
            }

            line.chars().enumerate().for_each(|(i, c)| {
                if c != '^' {
                    return;
                }

                if i > 0 {
                    timelines[i-1] += timelines[i];
                }



                if i < timelines.len()-1 {
                    timelines[i+1] += timelines[i];
                }

                timelines[i] = 0;
            });
        }
        return timelines.iter().sum();
    }

    0
}
