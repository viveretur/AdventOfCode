use std::io::{self, Read};

fn solve(line: &str) -> i32 {
    line.trim()
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line)?;

    let floor = solve(&line);
    println!("{floor}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("(())"), 0);
        assert_eq!(solve("()()"), 0);
        assert_eq!(solve("((("), 3);
        assert_eq!(solve("(()(()("), 3);
        assert_eq!(solve("))((((("), 3);
        assert_eq!(solve("())"), -1);
        assert_eq!(solve("))("), -1);
        assert_eq!(solve(")))"), -3);
        assert_eq!(solve(")())())"), -3);
    }
}
