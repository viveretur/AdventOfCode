use std::io::{self, Read};

fn solve(line: &str) -> Option<usize> {
    let mut floor = 0;

    for (pos, i) in line.bytes().enumerate() {
        floor += match i {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        };
        if floor < 0 {
            return Some(pos + 1);
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line)?;

    if let Some(floor) = solve(&line) {
        println!("{floor}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(")"), Some(1));
        assert_eq!(solve("()())"), Some(5));
    }
}
