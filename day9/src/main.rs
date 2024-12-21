use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty { length: usize },
    File { id: usize, length: usize },
}

struct Blockerator<'a> {
    inner: Box<dyn Iterator<Item = &'a Block> + 'a>,
    current: (usize, usize), // (id, length)
}

impl<'a> Blockerator<'a> {
    fn new(blocks: &'a [Block]) -> Self {
        Self {
            inner: Box::new(
                blocks
                    .iter()
                    .rev()
                    .filter(|b| matches!(b, Block::File { .. })),
            ),
            current: (0, 0),
        }
    }

    /// Take a bunch of blocks off the end of the stack, until the requested amount of bytes are
    /// fulfilled
    fn take(&mut self, mut bytes: usize) -> Vec<Block> {
        let mut out = vec![];
        while bytes > 0 {
            // Refresh the current block
            if self.current.1 == 0 {
                self.current = if let &Block::File { id, length } =
                    self.inner.next().expect("No more blocks!")
                {
                    (id, length)
                } else {
                    unreachable!("We ran out of blocks!")
                };
            }

            let bytes_taken = self.current.1.min(bytes);

            out.push(Block::File {
                id: self.current.0,
                length: bytes_taken,
            });

            self.current.1 -= bytes_taken;
            bytes -= bytes_taken;
        }

        out
    }
}

fn part1() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let (_, parsed) = input
        .trim()
        .chars()
        // Parse inputs as dgits
        .map(|c| c.to_digit(10).expect("Failed to parse digit"))
        .enumerate()
        // Convert inputs into structured blocks
        .fold((0, vec![]), |(mut id, mut parsed), (i, val)| {
            if i % 2 == 0 {
                // file
                parsed.push(Block::File {
                    id,
                    length: val as usize,
                });
                id += 1;
            } else {
                // not a file
                parsed.push(Block::Empty {
                    length: val as usize,
                })
            }

            (id, parsed)
        });

    println!("blocks: {}", parsed.len());
    println!("blocks ..5: {:?}", &parsed[..10]);
    println!("blocks -5..: {:?}", &parsed[parsed.len() - 10..]);

    let compressed_length = parsed
        .iter()
        .filter_map(|b| match b {
            Block::File { length, .. } => Some(*length),
            _ => None,
        })
        .sum::<usize>();

    println!("Compressed length: {}", compressed_length);

    let mut blockerator = Blockerator::new(&parsed);

    let (_, checksum) = parsed
        .iter()
        // Replace empty blocks with files from the end
        .flat_map(|b| match b {
            Block::Empty { length } => blockerator.take(*length),
            Block::File { .. } => vec![*b],
        })
        .fold((0, 0), |(mut total_bytes, mut checksum), x| {
            // If we've already processed enough files, no-op for the rest
            if total_bytes >= compressed_length {
                return (total_bytes, checksum);
            }

            // Calculate the checksum
            if let Block::File { id, length } = x {
                let length = length.min(compressed_length - total_bytes);
                checksum += (total_bytes..total_bytes + length)
                    .map(|pos| pos * id)
                    .sum::<usize>();
                total_bytes += length;
            } else {
                unreachable!()
            }

            (total_bytes, checksum)
        });
    println!("{:?}", checksum);
}

fn part2() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let (_, parsed) = input
        .trim()
        .chars()
        // Parse inputs as dgits
        .map(|c| c.to_digit(10).expect("Failed to parse digit"))
        .enumerate()
        // Convert inputs into structured blocks
        .fold((0, vec![]), |(mut id, mut parsed), (i, val)| {
            if i % 2 == 0 {
                // file
                parsed.push(Block::File {
                    id,
                    length: val as usize,
                });
                id += 1;
            } else {
                // not a file
                parsed.push(Block::Empty {
                    length: val as usize,
                })
            }

            (id, parsed)
        });
}

fn main() {
    part2();
}
