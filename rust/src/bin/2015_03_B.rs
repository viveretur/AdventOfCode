use std::collections::HashSet;
use std::io;

fn get_move(loc: (i32, i32), mov: u8) -> (i32, i32) {
    match mov {
        b'<' => (loc.0 - 1, loc.1),
        b'^' => (loc.0, loc.1 - 1),
        b'>' => (loc.0 + 1, loc.1),
        b'v' => (loc.0, loc.1 + 1),
        _ => loc,
    }
}

fn solve(data: &str) -> usize {
    let mut seen = HashSet::new();
    let mut santa = (0i32, 0i32);
    let mut robot = (0i32, 0i32);
    seen.insert(santa);
    for (i, b) in data.trim().bytes().enumerate() {
        if i % 2 == 0 {
            santa = get_move(santa, b);
            seen.insert(santa);
        } else {
            robot = get_move(robot, b);
            seen.insert(robot);
        }
    }
    seen.len()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("^v"), 3);
        assert_eq!(solve("^>v<"), 3);
        assert_eq!(solve("^v^v^v^v^v"), 11);
    }
}
