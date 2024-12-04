#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

fn part1() {
    let num_safe = std::io::stdin()
        // Parse inputs
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split(" ")
                .map(|x| u64::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>()
        })
        // Check if the report is safe
        .map(|r| check_report(&r))
        // Count them
        .filter(|s| *s == Safety::Safe)
        .count();
    println!("Safe reports: {}", num_safe);
}

/// Checks if a report is safe
fn check_report(report: &[u64]) -> Safety {
    let max_diff = report
        .windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .max()
        .unwrap();
    let ordering = report
        .windows(2)
        .map(|w| w[0].cmp(&w[1]))
        .collect::<Vec<_>>();

    if max_diff > 3 {
        Safety::Unsafe
    } else if ordering.windows(2).any(|w| w[0] != w[1]) {
        Safety::Unsafe
    } else {
        match ordering[0] {
            std::cmp::Ordering::Equal => Safety::Unsafe,
            _ => Safety::Safe,
        }
    }
}

fn part2_dumb() {
    let num_safe = std::io::stdin()
        // Parse inputs
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split(" ")
                .map(|x| u64::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>()
        })
        // Remove each of the levels one by one and measure the safety
        .map(|r| {
            (0..r.len())
                .map(|i| {
                    r[..i]
                        .iter()
                        .chain(r[i + 1..].iter())
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .map(|r| check_report(&r))
                .collect::<Vec<_>>()
        })
        // Check if any of the modified reports are safe
        .map(|reports| {
            reports
                .into_iter()
                .find(|s| *s == Safety::Safe)
                .unwrap_or(Safety::Unsafe)
        })
        // Count them
        .filter(|s| *s == Safety::Safe)
        .count();
    println!("Safe reports: {}", num_safe);
}

fn part2_smort() {
    let num_safe = std::io::stdin()
        // Parse inputs
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split(" ")
                .map(|x| u64::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>()
        })
        // Check if the report is safe
        .map(|mut r| {
            println!("{:?}", r);

            // Identify the first bad level
            let differences = r
                .windows(2)
                .map(|w| w[1] as i64 - w[0] as i64)
                .collect::<Vec<_>>();
            println!("\tdiffs: {:?}", differences);

            let signs = differences.iter().map(|d| d.cmp(&0)).collect::<Vec<_>>();
            println!("\tsigns: {:?}", signs);

            let first_bad = differences
                .iter()
                .zip(&signs)
                .enumerate()
                .find(|&(_, (d, s))| {
                    *s == std::cmp::Ordering::Equal || *s != signs[0] || d.abs() > 3
                });
            println!("\tfirst_bad: {:?}", first_bad);

            // remove it
            // todo: we need to decide whether to remove this one, or the one before it
            if let Some((i, _)) = first_bad {
                let mut r1 = r.clone();
                r1.remove(i);
                println!("\trem first: {:?}", check_report(&r1));

                let mut r2 = r.clone();
                r2.remove(i + 1);
                println!("\trem 2nd: {:?}", check_report(&r2));
            };

            check_report(&r)
        })
        // Count them
        .filter(|s| *s == Safety::Safe)
        .count();
    println!("Safe reports: {}", num_safe);
}

fn main() {
    part2_dumb();
}
