use std::cmp::Reverse;
use std::io;

fn solve(input: &str) -> u32 {
    let mut groups = input
        .split("\n\n")
        .map(|g| g.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    groups.sort_unstable_by_key(|x| Reverse(*x));

    groups[..3].iter().sum()
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

        let data = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        assert_eq!(solve(data), 45000);
    }
}
