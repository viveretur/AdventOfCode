use std::collections::HashSet;
use std::io::{self, Read};

fn solve(line: &str) -> i32 {
    let deltas: Vec<i32> = line
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut seen = HashSet::new();
    let mut freq = 0i32;

    for d in deltas.iter().copied().cycle() {
        if !seen.insert(freq) {
            return freq;
        }
        freq += d;
    }

    unreachable!()
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

        assert_eq!(solve("+1\n-1"), 0);
        assert_eq!(solve("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(solve("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(solve("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
