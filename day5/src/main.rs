use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn part2() {
    let input = std::io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read inputs");
    let input = input.split(|s| s.is_empty()).collect::<Vec<_>>();

    let disqualifiers = input[0]
        .iter()
        // Split each constraint
        .map(|s| s.split_once("|").expect("Failed to split string"))
        // Construct a LUT which flags immediate fails
        .fold(
            HashMap::<_, HashSet<_>>::new(),
            |mut hm, (before, after)| {
                hm.entry(before).or_default().insert(after);
                hm
            },
        );

    let total = input[1]
        .iter()
        // Parse each test into pages
        .map(|s| s.split(',').collect::<Vec<_>>())
        // Get rid of passed tests
        .filter(|test| {
            // Check that all pages follow the right ordering rules
            !test.iter().enumerate().all(|(i, page)| {
                // Make sure that there's no previous pages that would break the ordering rule
                disqualifiers
                    .get(page)
                    .map(|disallowed_pages| {
                        disallowed_pages.is_disjoint(&test[..i].iter().copied().collect())
                    })
                    .unwrap_or(true)
            })
        })
        // Fix the ordering of the pages by sorting them according to their constraints
        .map(|test| {
            let mut test = test.clone();
            test.sort_by(|a, b| {
                let a_in_b = disqualifiers
                    .get(a)
                    .map(|hs| hs.contains(b))
                    .unwrap_or_default();
                let b_in_a = disqualifiers
                    .get(b)
                    .map(|hs| hs.contains(a))
                    .unwrap_or_default();
                match (a_in_b, b_in_a) {
                    (true, true) => unreachable!("Can't have dual constraints!"),
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (false, false) => Ordering::Equal,
                }
            });

            test
        })
        // Grab the middle page as an integer
        .map(|test| {
            test[test.len() / 2]
                .parse::<u64>()
                .expect("Failed to parse string as int")
        })
        // Add em up
        .sum::<u64>();

    println!("Total: {}", total);
}

fn part1() {
    let input = std::io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read inputs");
    let input = input.split(|s| s.is_empty()).collect::<Vec<_>>();

    let disqualifiers = input[0]
        .iter()
        // Split each constraint
        .map(|s| s.split_once("|").expect("Failed to split string"))
        // Construct a LUT which flags immediate fails
        .fold(
            HashMap::<_, HashSet<_>>::new(),
            |mut hm, (before, after)| {
                hm.entry(before).or_default().insert(after);
                hm
            },
        );

    let total = input[1]
        .iter()
        // Parse each test into pages
        .map(|s| s.split(',').collect::<Vec<_>>())
        // Get rid of failed tests
        .filter(|test| {
            // Check that all pages follow the right ordering rules
            test.iter().enumerate().all(|(i, page)| {
                // Make sure that there's no previous pages that would break the ordering rule
                disqualifiers
                    .get(page)
                    .map(|disallowed_pages| {
                        disallowed_pages.is_disjoint(&test[..i].iter().copied().collect())
                    })
                    .unwrap_or(true)
            })
        })
        // Grab the middle page as an integer
        .map(|test| {
            test[test.len() / 2]
                .parse::<u64>()
                .expect("Failed to parse string as int")
        })
        // Add em up
        .sum::<u64>();

    println!("Total: {}", total);
}

fn main() {
    part2()
}
