use std::usize;

/// Counts how many times the given pattern exists at the given point, in any direction
fn grid_regex(grid: &[Vec<char>], pattern: &[char], grid_point: (usize, usize)) -> usize {
    let step_sizes = [-1, 0, 1];

    step_sizes
        .iter()
        // Generate 8 directions
        .flat_map(|i| step_sizes.iter().map(|j| (i, j)).collect::<Vec<_>>())
        .filter(|(i, j)| **i != 0 || **j != 0)
        .map(|(i, j)| {
            // Test if the pattern matches along this direction
            pattern.iter().enumerate().all(|(d, pattern_letter)| {
                // If we're out of bounds, fail the match
                let y = grid_point.0 as isize + d as isize * *i;
                let x = grid_point.1 as isize + d as isize * *j;
                if y < 0 || x < 0 {
                    return false;
                }

                // Grab the char from the grid and test it
                grid.get(y as usize)
                    .and_then(|row| row.get(x as usize))
                    .map_or(false, |test_letter| test_letter == pattern_letter)
            })
        })
        // Count matches
        .filter(|m| *m)
        .count()
}

fn part1() {
    let regex = "XMAS".chars().collect::<Vec<_>>();

    let grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total_matches = grid
        .iter()
        .enumerate()
        // Generate (y, x) points
        .flat_map(|(i, row)| (0..row.len()).map(|j| (i, j)).collect::<Vec<_>>())
        // Test the regex at this point
        .map(|point| grid_regex(&grid, &regex, point))
        .sum::<usize>();

    println!("Matches: {}", total_matches)
}

/// Rotates the kernel 90 degrees clockwise
fn rotate_kernel(kernel: &[Vec<Option<char>>]) -> Vec<Vec<Option<char>>> {
    (0..kernel[0].len())
        .map(|i| {
            (0..kernel.len())
                .rev()
                .map(|j| kernel[j][i])
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Tests the kernel against the grid to see if it matches
fn test_kernel(grid: &[&[char]], kernel: &[Vec<Option<char>>]) -> bool {
    grid.iter()
        // Loop over the grid & kernel in pairs
        .flat_map(|row| row.iter())
        .zip(kernel.iter().flatten())
        // Compare the chars, None in the kernel counts as a wildcard
        .all(|(test_letter, pattern_letter)| {
            pattern_letter.map(|p| *test_letter == p).unwrap_or(true)
        })
}

/// Counts how many matches the kernel gets on this grid section
fn kernel_regex(grid: &[&[char]], kernel: &[Vec<Option<char>>]) -> usize {
    // Generate 4 different orientations of kernels and test them against the grid
    let (_, matches) = (0..4).fold((kernel.to_vec(), 0), |(k, count), _| {
        let count = if test_kernel(grid, &k) {
            count + 1
        } else {
            count
        };

        (rotate_kernel(&k), count)
    });

    matches
}

fn part2() {
    // Grab the input grid
    let grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Kernel used to match against. None counts as a wildcard
    let kernel = vec![
        vec![Some('M'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('S')],
    ];

    let total_matches = (0..grid.len() - kernel.len() + 1)
        // Generate (y, x) coordinates
        .flat_map(|i| {
            (0..grid[0].len() - kernel[0].len() + 1)
                .map(|j| (i, j))
                .collect::<Vec<_>>()
        })
        // Create a grid slice in this location
        .map(|(i, j)| {
            grid[i..i + kernel.len()]
                .iter()
                .map(|row| &row[j..j + kernel.len()])
                .collect::<Vec<_>>()
        })
        // Test the kernel on the grid slice
        .map(|grid_section| kernel_regex(&grid_section, &kernel))
        // Add em up
        .sum::<usize>();

    println!("Total matches: {}", total_matches);
}

fn main() {
    part2()
}
