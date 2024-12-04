use std::collections::HashMap;

fn main() {
    // Parse input strings
    let pairs = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split("   ")
                .map(|x| u64::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // Pull apart the two lists
    let mut list1 = pairs.iter().step_by(2).copied().collect::<Vec<_>>();
    let mut list2 = pairs.iter().skip(1).step_by(2).copied().collect::<Vec<_>>();
    list1.sort();
    list2.sort();

    // Calculate the total
    let total = list1
        .iter()
        .zip(&list2)
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u64>();

    println!("Total: {}", total);

    // Count values in right list
    let counts = list2.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;

        acc
    });

    // Calculate the similarity score
    let similarity_score = list1
        .iter()
        .map(|x| x * counts.get(x).unwrap_or(&0))
        .sum::<u64>();

    println!("similarity_score: {}", similarity_score);
}
