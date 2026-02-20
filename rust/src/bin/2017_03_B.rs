use std::collections::HashMap;
use std::io;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_left(self) -> Self {
        use Direction::*;

        match self {
            N => W,
            E => N,
            S => E,
            W => S,
        }
    }

    fn delta(self) -> (i32, i32) {
        use Direction::*;

        match self {
            E => (1, 0),
            N => (0, 1),
            W => (-1, 0),
            S => (0, -1),
        }
    }
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn solve(val: i32) -> i32 {
    const NEIGHBOURS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut loc = (0i32, 0i32);
    let mut dir = Direction::E;

    map.insert(loc, 1);
    let mut current = 1;

    while current <= val {
        let left_dir = dir.turn_left();
        let left_loc = add(loc, left_dir.delta());

        if !map.contains_key(&left_loc) {
            dir = left_dir;
        }
        loc = add(loc, dir.delta());

        current = NEIGHBOURS
            .iter()
            .map(|&(dx, dy)| map.get(&add(loc, (dx, dy))).copied().unwrap_or(0))
            .sum();

        map.insert(loc, current);
        #[cfg(debug_assertions)]
        println!("{},{} -> {}", loc.0, loc.1, current);
    }

    current
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?
        .trim()
        .parse::<i32>()
        .unwrap();
    println!("{}", solve(data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(100), 122);
        assert_eq!(solve(1), 2);
        assert_eq!(solve(500), 747);
    }
}
