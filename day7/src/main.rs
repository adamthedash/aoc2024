use itertools::{repeat_n, Itertools};

/// Checks if it is possible to create the total
fn check_case(total: u64, operands: &[u64]) -> bool {
    let operators = [
        // +
        |a, b| a + b,
        // *
        |a, b| a * b,
        // ||
        |a, b| {
            // Calculate how many digits we need to shift left
            let b_log10 = ((b + 1) as f64).log10().ceil();
            let mult = 10_u64.pow(b_log10 as u32);

            a * mult + b
        },
    ];

    // Iterate over every combination of applying the operators
    repeat_n(operators, operands.len() - 1)
        .multi_cartesian_product()
        .any(|funcs| {
            // Reduce the numbers & check if it matches the total
            operands.iter().enumerate().fold(
                0,
                |acc, (i, x)| {
                    if i == 0 {
                        *x
                    } else {
                        funcs[i - 1](acc, *x)
                    }
                },
            ) == total
        })
}

fn main() {
    // Read the lines in
    let total = std::io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input")
        .into_iter()
        // Parse out the numbers
        .map(|l| {
            let (a, b) = l.split_once(": ").expect("Failed to split string");
            let total = a.parse::<u64>().expect("Failed to parse total");
            let operands = b
                .split(" ")
                .map(|x| x.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .expect("Failed to parse operands");

            (total, operands)
        })
        // Filter out only ones that are possible
        .filter(|(total, operands)| check_case(*total, operands.as_slice()))
        // Sum up the totals
        .map(|(total, _)| total)
        .sum::<u64>();

    println!("Total: {}", total);
}
