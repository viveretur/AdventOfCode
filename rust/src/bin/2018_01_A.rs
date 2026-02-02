use std::io::{self, Read};

fn solve(line: &str) -> i32 {
    line.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line)?;

    let result = solve(&line.trim());
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        assert_eq!(solve("+1\n-2\n+3\n+1"), 3);
        assert_eq!(solve("+1\n+1\n+1"), 3);
        assert_eq!(solve("+1\n+1\n-2"), 0);
        assert_eq!(solve("-1\n-2\n-3"), -6);
    }
}
