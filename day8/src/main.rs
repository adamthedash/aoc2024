#[macro_use]
extern crate impl_ops;
use std::ops;

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point(isize, isize);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl_op_ex!(+ |p1: &Point, p2: &Point| -> Point { Point(p2.0 + p1.0, p2.1 + p1.1) });
impl_op_ex!(-|p1: &Point, p2: &Point| -> Point { Point(p1.0 - p2.0, p1.1 - p2.1) });
impl_op_ex!(*|p: &Point, mul: usize| -> Point { Point(p.0 * mul as isize, p.1 * mul as isize) });

impl Point {
    /// Checks if the point is within the bounds of the map
    fn is_within_bounds(&self, bounds: (usize, usize)) -> bool {
        self.0 >= 0 && self.0 < bounds.0 as isize && self.1 >= 0 && self.1 < bounds.1 as isize
    }
}

fn part1() {
    // Read input & get map size
    let input = std::io::stdin()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect::<Vec<_>>();
    let map_size = (input.len(), input[0].len());

    // Parse the antennae into their locations, partitioned by their identifier
    let nodes = input
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, Point::from((y, x))))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut hm, (c, pos)| {
            hm.entry(c).or_default().push(pos);
            hm
        });

    // Generate the antinodes for each pair of antennae
    let antinodes = nodes
        .iter()
        .map(|(c, positions)| {
            let antinodes = positions
                .iter()
                .permutations(2)
                .map(|points| points[1] + points[1] - points[0])
                .filter(|p| p.is_within_bounds(map_size))
                .collect::<Vec<_>>();

            (c, antinodes)
        })
        .collect::<HashMap<_, _>>();

    // Count how many unique antinodes there are
    let mut unique_antinodes = antinodes.values().flatten().cloned().collect::<Vec<_>>();
    unique_antinodes.sort();
    unique_antinodes.dedup();

    println!("{:?}", unique_antinodes.len())
}

/// Geneerates all the antinodes for two antennae in a line
fn generate_antinodes(p1: &Point, p2: &Point, map_bounds: (usize, usize)) -> Vec<Point> {
    let diff = p2 - p1;

    let left = (0..)
        .map(|distance| p2 + &diff * distance)
        .take_while(|p| p.is_within_bounds(map_bounds));

    let right = (0..)
        .map(|distance| p1 - &diff * distance)
        .take_while(|p| p.is_within_bounds(map_bounds));

    left.chain(right).collect()
}

fn part2() {
    // Read input & get map size
    let input = std::io::stdin()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect::<Vec<_>>();
    let map_size = (input.len(), input[0].len());
    println!("Map size: {:?}", map_size);

    // Parse the antennae into their locations, partitioned by their identifier
    let nodes = input
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            // Create Point objects
            l.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, Point::from((y, x))))
                .collect::<Vec<_>>()
        })
        // Collect into frequency -> [location] map
        .fold(HashMap::<_, Vec<_>>::new(), |mut hm, (c, pos)| {
            hm.entry(c).or_default().push(pos);
            hm
        });

    // Generate the antinodes for each pair of antennae
    let antinodes = nodes
        .iter()
        .map(|(c, positions)| {
            // Iterate over each pair of antennae
            let antinodes = positions
                .iter()
                .combinations(2)
                // Generate all the antinodes
                .flat_map(|points| generate_antinodes(points[0], points[1], map_size))
                .collect::<Vec<_>>();

            (c, antinodes)
        })
        .collect::<HashMap<_, _>>();

    // Count how many unique antinodes there are
    let mut unique_antinodes = antinodes.values().flatten().cloned().collect::<Vec<_>>();
    unique_antinodes.sort();
    unique_antinodes.dedup();

    println!("{:?}", unique_antinodes.len())
}

fn main() {
    part2();
}
