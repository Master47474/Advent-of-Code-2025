
#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    let mut numbers = vec!();
    let mut row_count = 0;
    input.lines().for_each(|line| {
        row_count += 1;

        line.split(' ').for_each(|n_str| {
            if (n_str.is_empty()){
                return;
            }
            numbers.push(n_str);
        });
    });

    let col_max = numbers.len() / row_count;

    let mut sum = 0;
    for col in 0..col_max {
        let operator = numbers[(col_max * (row_count-1)) + col];
        let mut mini_answer = numbers[col].parse::<u64>().unwrap();
        for r in 1..row_count-1 {
            let n = numbers[(col_max * r) + col].parse::<u64>().unwrap();
            match operator {
                "*" => mini_answer *= n,
                "+" => mini_answer += n,
                _ => {}
            }
        }
        sum += mini_answer;
    }

    sum
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
  0
}
