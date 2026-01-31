use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn(self, t: Turn) -> Self {
        use Direction::*;
        use Turn::*;

        match (self, t) {
            (N, Left) => W,
            (E, Left) => N,
            (S, Left) => E,
            (W, Left) => S,
            (N, Right) => E,
            (E, Right) => S,
            (S, Right) => W,
            (W, Right) => N,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Direction::N => (0, 1),
            Direction::E => (1, 0),
            Direction::S => (0, -1),
            Direction::W => (-1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(self, dir: Direction, amt: i32) -> Self {
        let (dx, dy) = dir.delta();
        Self {
            x: self.x + dx * amt,
            y: self.y + dy * amt,
        }
    }

    fn manhattan(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn solve(line: &str) -> i32 {
    let mut pos = Pos::default();
    let mut direction = Direction::N;
    let mut seen: HashSet<Pos> = HashSet::new();
    seen.insert(pos);

    'find: for key in line
        .trim()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let mut it = key.chars();
        let turn = match it.next().unwrap() {
            'R' => Turn::Right,
            'L' => Turn::Left,
            _ => unreachable!(),
        };
        let amt: i32 = it.as_str().parse().unwrap();

        direction = direction.turn(turn);
        for _ in 0..amt {
            pos = pos.step(direction, 1);
            if !seen.insert(pos) {
                break 'find;
            }
        }
    }

    pos.manhattan()
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line)?;

    let distance = solve(&line);
    println!("{distance}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("R8, R4, R4, R8"), 4);
    }
}
