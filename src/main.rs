mod days;

fn main() {
    let input = include_str!("../input/day4.txt");

    let start = std::time::Instant::now();

    let result = days::day3::part2(input);

    let duration = start.elapsed();

    println!("answer: {result}\nduration: {duration:?}");
}
