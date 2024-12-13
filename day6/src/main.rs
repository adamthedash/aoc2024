use itertools::{FoldWhile, Itertools};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
}

impl Position {
    /// Goes backwards one step, if possible
    fn move_back(&self, map_size: (usize, usize)) -> Option<Self> {
        let diff = match self.facing {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };

        // Move & bounds check
        let new_y = self.y.checked_add_signed(diff.0)?;
        let new_x = self.x.checked_add_signed(diff.1)?;
        if new_y >= map_size.0 {
            return None;
        }
        if new_x >= map_size.1 {
            return None;
        }

        Some(Self {
            y: new_y,
            x: new_x,
            facing: self.facing,
        })
    }

    /// Goes left one step if possible
    fn move_left(&self, map_size: (usize, usize)) -> Option<Self> {
        let diff = match self.facing {
            Direction::Up => (0, -1),
            Direction::Right => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (1, 0),
        };

        // Move & bounds check
        let new_y = self.y.checked_add_signed(diff.0)?;
        let new_x = self.x.checked_add_signed(diff.1)?;
        if new_y >= map_size.0 {
            return None;
        }
        if new_x >= map_size.1 {
            return None;
        }

        Some(Self {
            y: new_y,
            x: new_x,
            ..*self
        })
    }

    fn turn_left(&self) -> Self {
        Self {
            facing: self.facing.turn_left(),
            ..*self
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
    facing: Direction,
}

fn part1() {
    let input = std::io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read input");
    let map_height = input.len();
    let map_width = input[0].len();

    // Load all the objects
    let objects = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (y, x, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find the guard
    let guard = objects
        .iter()
        .find(|(_, _, c)| *c == '^')
        .map(|&(y, x, _)| Position {
            y,
            x,
            facing: Direction::Up,
        })
        .expect("Couldn't find the guard!");

    // Pull out the obstacles
    let obstacles = objects
        .into_iter()
        .filter(|(_, _, c)| *c == '#')
        .map(|(y, x, _)| (y, x))
        .collect::<Vec<_>>();

    let (mut steps, guard) = (0..)
        .fold_while(
            (vec![(guard.y, guard.x)], guard),
            |(mut steps, mut guard), _| {
                let next_bonk = obstacles
                    .iter()
                    // Filer only obstacles that are in front of the guard
                    .filter(|&&(y, x)| match guard.facing {
                        Direction::Up => y < guard.y && x == guard.x,
                        Direction::Right => y == guard.y && x > guard.x,
                        Direction::Down => y > guard.y && x == guard.x,
                        Direction::Left => y == guard.y && x < guard.x,
                    })
                    // And then get the closest
                    .min_by_key(|(y, x)| y.abs_diff(guard.y) + x.abs_diff(guard.x))
                    .copied();

                // If there's no obstacle left, note it and put a fake one down
                let is_finished = next_bonk.is_none();
                let (y, x) = next_bonk.unwrap_or_else(|| match guard.facing {
                    Direction::Up => (0, guard.x),
                    Direction::Right => (guard.y, map_width - 1),
                    Direction::Down => (map_height - 1, guard.x),
                    Direction::Left => (guard.y, 0),
                });

                // Generate the list of steps the guard will take this time
                let guard_steps: Vec<_> = match guard.facing {
                    Direction::Up => (y + 1..guard.y).rev().map(|y| (y, guard.x)).collect(),
                    Direction::Right => (guard.x + 1..x).map(|x| (guard.y, x)).collect(),
                    Direction::Down => (guard.y + 1..y).map(|y| (y, guard.x)).collect(),
                    Direction::Left => (x + 1..guard.x).rev().map(|x| (guard.y, x)).collect(),
                };
                // And add them to the list we're folding up
                steps.extend(guard_steps.iter());

                // Update the guard
                guard.facing = guard.facing.turn_right();
                (guard.y, guard.x) = steps[steps.len() - 1];

                // If we're finished, also add the final step where the fake obstacle is
                if is_finished {
                    steps.push((y, x));
                }

                if is_finished {
                    FoldWhile::Done((steps, guard))
                } else {
                    FoldWhile::Continue((steps, guard))
                }
            },
        )
        .into_inner();

    // Deduplicate steps
    steps.sort();
    steps.dedup();
    println!("steps: {}", steps.len());
}

/// Recursively walk back from the given point checking for vaiable obstacle positions
fn walk_back(pos: &Position, turning_points: Vec<Position>, obstacles: Vec<(usize, usize)>) {
    // Scenario 1: There is no obstacle to our left
    //      Do nothing
    // Scenario 2: There is an obstacle to our left
    //      Turn left, and recurse
    //
    // Scenario A: There is nothing behind us
    //      Move backwards
    // Scenario B: There is something behind us (obstacle or map edge)
    //      Terminate
    //
    //  Scenario I: We encounter a future step, facing left of us
    //      Drop an obstacle to our left
    //  Scenario II:
}

fn part2() {
    let input = std::io::stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read input");
    let map_height = input.len();
    let map_width = input[0].len();

    // Load all the objects
    let objects = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (y, x, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find the guard
    let guard = objects
        .iter()
        .find(|(_, _, c)| *c == '^')
        .map(|&(y, x, _)| Position {
            y,
            x,
            facing: Direction::Up,
        })
        .expect("Couldn't find the guard!");

    // Pull out the obstacles
    let obstacles = objects
        .into_iter()
        .filter(|(_, _, c)| *c == '#')
        .map(|(y, x, _)| (y, x))
        .collect::<Vec<_>>();

    let (steps, guard, turning_points) = (0..)
        .fold_while(
            (vec![(guard.y, guard.x)], guard, vec![]),
            |(mut steps, mut guard, mut turning_points), _| {
                let next_bonk = obstacles
                    .iter()
                    // Filer only obstacles that are in front of the guard
                    .filter(|&&(y, x)| match guard.facing {
                        Direction::Up => y < guard.y && x == guard.x,
                        Direction::Right => y == guard.y && x > guard.x,
                        Direction::Down => y > guard.y && x == guard.x,
                        Direction::Left => y == guard.y && x < guard.x,
                    })
                    // And then get the closest
                    .min_by_key(|(y, x)| y.abs_diff(guard.y) + x.abs_diff(guard.x))
                    .copied();

                // If there's no obstacle left, note it and put a fake one down
                let is_finished = next_bonk.is_none();
                let (y, x) = next_bonk.unwrap_or_else(|| match guard.facing {
                    Direction::Up => (0, guard.x),
                    Direction::Right => (guard.y, map_width - 1),
                    Direction::Down => (map_height - 1, guard.x),
                    Direction::Left => (guard.y, 0),
                });

                // Generate the list of steps the guard will take this time
                let guard_steps: Vec<_> = match guard.facing {
                    Direction::Up => (y + 1..guard.y).rev().map(|y| (y, guard.x)).collect(),
                    Direction::Right => (guard.x + 1..x).map(|x| (guard.y, x)).collect(),
                    Direction::Down => (guard.y + 1..y).map(|y| (y, guard.x)).collect(),
                    Direction::Left => (x + 1..guard.x).rev().map(|x| (guard.y, x)).collect(),
                };

                // And add them to the list we're folding up
                steps.extend(guard_steps.iter());

                // Update the guard
                (guard.y, guard.x) = guard_steps[guard_steps.len() - 1];
                turning_points.push(guard);
                guard.facing = guard.facing.turn_right();

                // If we're finished, also add the final step where the fake obstacle is
                if is_finished {
                    steps.push((y, x));
                }

                if is_finished {
                    FoldWhile::Done((steps, guard, turning_points))
                } else {
                    FoldWhile::Continue((steps, guard, turning_points))
                }
            },
        )
        .into_inner();

    println!("Steps: {:?}", steps.len());

    // Constraints for part 2:
    // 1) Obstacle must be placed on a tile that a guard walks on, except the first one
    // 2) To detect a loop, a guard must either:
    //      a) Walk onto a previously stepped on tile, facing the same direction
    //      b) Walk onto a previously stepped on tile, facing 90 degrees left from where they were
    //      before, with an obstacle in front of them
    //
    // Possible solutions:
    // 1) Brute force - place an obstacle at every step, and re-run the simulation
    // 2) Walk back:
    //      a) Start at each existing turning point
    //      b) Walk backwards, looking for obstacles to the left
    //      c) If we've already turned at this point, ignore it
    //      d) If we've not, branch into two and recurse - one turns left, the other continues
    //      straight
    //      e) Continue walking backwards until either we reach the map edge (no-viable obstacle
    //      placements), or we run into a previous step facing left (viable obstacle location).

    println!("Turning points: {:?}", turning_points);

    turning_points
        .into_iter()
        .map(|p| {
            let is_branching_point = p
                .move_left((map_height, map_width))
                .map(|l| obstacles.contains(&(l.y, l.x)));

            if is_branching_point.is_some_and(|b| b) {
                println!("Branch: {:?}", p);
                if turning_points.contains(&p.turn_left()) {
                    // Do nothing
                } else {
                    // Branch & recurse
                    let p1 = p.move_back((map_height, map_width));
                    let p2 = p.turn_left();
                }
            }
        })
        .count();
}

fn main() {
    part2();
}
