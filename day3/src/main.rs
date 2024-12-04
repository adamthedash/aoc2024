use regex::Regex;
use std::io::Read;

#[derive(Debug)]
enum Command {
    Do,
    Dont,
    Mul(u64, u64),
}

fn part2() {
    // Matches "mul(a, b)", "do()", "don't()"
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    // Read input
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (_, sum) = re
        // Apply regex
        .captures_iter(input.as_str())
        // Extract capture groups
        .map(|c| {
            c.iter()
                .collect::<Vec<_>>()
                .try_into()
                .expect("failed to fit regex into array")
        })
        // Parse capture groups into Command enum
        .map(|c: [_; 5]| match c {
            [_, Some(a), Some(b), None, None] => Command::Mul(
                a.as_str().parse::<u64>().expect("Failed to parse mul lhs"),
                b.as_str().parse::<u64>().expect("Failed to parse mul rhs"),
            ),
            [_, None, None, Some(_), None] => Command::Do,
            [_, None, None, None, Some(_)] => Command::Dont,
            _ => panic!("Match arm fucked"),
        })
        // Accumulate commands
        .fold((true, 0), |(enabled, sum), c| match (c, enabled) {
            (Command::Do, _) => (true, sum),
            (Command::Dont, _) => (false, sum),
            (Command::Mul(a, b), true) => (enabled, sum + a * b),
            (Command::Mul(_, _), false) => (enabled, sum),
        });

    println!("Sum: {}", sum);
}

fn part1() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let sum = re
        .captures_iter(input.as_str())
        .inspect(|c| println!("{:?}", c))
        .map(|c| c.extract())
        .map(|(_, nums)| nums.map(|x| x.parse::<u64>().expect("Failed to parse number")))
        .map(|[a, b]| a * b)
        .sum::<u64>();

    println!("Sum: {}", sum);
}

fn main() {
    part2();
}
