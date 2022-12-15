use anyhow::Result;
use std::{collections::HashSet, fs};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Motion {
    dir: Direction,
    distance: i32,
}

// Possible positions of the tail where it needs to move.
// The knot ahead in line can move diagonally so we need to
// account for those new possible positions.
//
// xxxxx
// x...x
// x.H.x
// x...x
// xxxxx
//
// There are only sixteen, we should simply enumerate them
// Let's use a two dimensional bit mask.
// There are five horizontal positions and five vertical positions.
//
// Lets use a i16, we only need 10 bits so that is enough
// To keep it simple we will use a single u8 for each dimension
// with the horizontal bit being the left most bit.
//
//   01234
// 0 xxxxx
// 1 x...x
// 2 x.H.x
// 3 x...x
// 4 xxxxx
//
//
//
//   01234
// 0 .....
// 1 .....      x         y
// 2 x.H..  0000_0000 0000_0100
// 3 .....
// 4 .....
//
//   01234
// 0 .....
// 1 .....      x         y
// 2 ..H..  0000_1000 0001_0000
// 3 .....
// 4 ...x.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn follow(&self, head: &Position) -> Position {
        let delta = self - head;
        let mask = delta.to_bitmask();
        let mv = match mask {
            // Organized clockwise starting at top left (-2,2)
            0b0000_0000_0000_0000 => Position { x: 1, y: 1 },

            0b0000_0010_0000_0000 => Position { x: 1, y: 1 },
            0b0000_0100_0000_0000 => Position { x: 0, y: 1 },
            0b0000_1000_0000_0000 => Position { x: -1, y: 1 },

            0b0001_0000_0000_0000 => Position { x: -1, y: 1 },

            0b0001_0000_0000_0010 => Position { x: -1, y: 1 },
            0b0001_0000_0000_0100 => Position { x: -1, y: 0 },
            0b0001_0000_0000_1000 => Position { x: -1, y: -1 },

            0b0001_0000_0001_0000 => Position { x: -1, y: -1 },

            0b0000_1000_0001_0000 => Position { x: -1, y: -1 },
            0b0000_0100_0001_0000 => Position { x: 0, y: -1 },
            0b0000_0010_0001_0000 => Position { x: 1, y: -1 },

            0b0000_0000_0001_0000 => Position { x: 1, y: -1 },

            0b0000_0000_0000_1000 => Position { x: 1, y: -1 },
            0b0000_0000_0000_0100 => Position { x: 1, y: 0 },
            0b0000_0000_0000_0010 => Position { x: 1, y: 1 },
            // Doesn't need to move
            _ => Position::default(),
        };
        self + &mv
    }
    fn to_bitmask(&self) -> u16 {
        let x: u16 = match self.x {
            -2 => 0b00000000,
            -1 => 0b00000010,
            0 => 0b00000100,
            1 => 0b00001000,
            2 => 0b00010000,
            _ => unreachable!("X: {}", self.x),
        };
        let y: u16 = match self.y {
            -2 => 0b00000000,
            -1 => 0b00000010,
            0 => 0b00000100,
            1 => 0b00001000,
            2 => 0b00010000,
            _ => unreachable!("Y: {}", self.y),
        };

        x << 8 | y
    }
}

impl std::ops::Sub<&Position> for &Position {
    type Output = Position;

    fn sub(self, rhs: &Position) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn mv(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => {
                self.knots[0].x -= 1;
            }
            Direction::Right => {
                self.knots[0].x += 1;
            }
            Direction::Up => {
                self.knots[0].y -= 1;
            }
            Direction::Down => {
                self.knots[0].y += 1;
            }
        };
        for i in 1..self.knots.len() {
            self.knots[i] = self.knots[i].follow(&self.knots[i - 1]);
        }
    }
    fn tail(&self) -> Position {
        self.knots[self.knots.len() - 1].clone()
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    // Fold all lines into a count
    let motions = input.lines().map(|l| {
        let parts = l.split(' ').collect::<Vec<&str>>();
        assert_eq!(2, parts.len());
        let dir = match parts[0] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };
        let distance = parts[1].parse().unwrap();
        Motion { dir, distance }
    });
    let mut positions = HashSet::<Position>::new();
    let mut rope = Rope {
        knots: Vec::with_capacity(10),
    };
    rope.knots.resize(10, Position::default());
    positions.insert(rope.tail());
    for m in motions {
        for _ in 0..m.distance {
            rope.mv(&m.dir);
            positions.insert(rope.tail());
        }
    }

    println!("Count: {}", positions.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all_moves() {
        // Doesn't move
        assert_eq!(
            Position { x: 11, y: 11 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 11, y: 11 }
        );
        // Organized clockwise starting at top left -1,-1 relative to head
        assert_eq!(
            Position { x: 9, y: 8 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 9 }
        );
        assert_eq!(
            Position { x: 10, y: 8 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 9 }
        );
        assert_eq!(
            Position { x: 11, y: 8 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 9 }
        );

        assert_eq!(
            Position { x: 12, y: 9 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 11, y: 10 }
        );
        assert_eq!(
            Position { x: 12, y: 10 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 11, y: 10 }
        );
        assert_eq!(
            Position { x: 12, y: 11 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 11, y: 10 }
        );

        assert_eq!(
            Position { x: 11, y: 12 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 11 }
        );
        assert_eq!(
            Position { x: 10, y: 12 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 11 }
        );
        assert_eq!(
            Position { x: 9, y: 12 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 10, y: 11 }
        );

        assert_eq!(
            Position { x: 8, y: 11 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 9, y: 10 }
        );
        assert_eq!(
            Position { x: 8, y: 10 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 9, y: 10 }
        );
        assert_eq!(
            Position { x: 8, y: 9 }.follow(&Position { x: 10, y: 10 }),
            Position { x: 9, y: 10 }
        );
    }
}
