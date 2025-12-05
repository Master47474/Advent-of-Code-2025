mod days;

fn main() {
    let input = include_str!("../input/day4.txt");

    let start = std::time::Instant::now();

    let result = days::day4::part1(input);

    let duration = start.elapsed();

    println!("answer: {result}\nduration: {duration:?}");
}
