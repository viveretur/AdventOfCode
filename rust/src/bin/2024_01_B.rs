use std::collections::HashMap;
use std::io;

fn solve(input: &str) -> i64 {
    let mut it = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let mut left = Vec::new();
    let mut right = HashMap::new();

    while let (Some(l), Some(r)) = (it.next(), it.next()) {
        left.push(l);
        *right.entry(r).or_insert(0) += 1;
    }

    left.into_iter()
        .map(|n| n * right.get(&n).copied().unwrap_or(0))
        .sum()
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("{}", solve(&input));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        let data = r#"3    4
4    3
2    5
1    3
3    9
3    3"#;

        assert_eq!(solve(data), 31);
    }
}
