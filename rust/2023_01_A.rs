use std::io;

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line
                .bytes()
                .filter_map(|c| (c.is_ascii_digit()).then_some((c - b'0') as u32));

            let first = it.next().expect("non vacua");
            let last = it.last().unwrap_or(first);
            first * 10 + last
        })
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

        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(solve(input), 142);
    }
}
