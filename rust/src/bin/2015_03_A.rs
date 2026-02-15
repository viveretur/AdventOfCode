use std::collections::HashSet;
use std::io;

fn solve(data: &str) -> usize {
    let mut seen = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    seen.insert((x, y));
    for b in data.trim().bytes() {
        match b {
            b'<' => x -= 1,
            b'^' => y -= 1,
            b'>' => x += 1,
            b'v' => y += 1,
            _ => (),
        }
        seen.insert((x, y));
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
        assert_eq!(solve(">"), 2);
        assert_eq!(solve("^>v<"), 4);
        assert_eq!(solve("^v^v^v^v^v"), 2);
    }
}
