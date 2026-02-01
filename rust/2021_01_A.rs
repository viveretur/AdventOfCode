use std::io;

fn solve(input: &str) -> u32 {
    let data = input.lines().map(|s| s.parse::<u32>().unwrap());

    data.clone()
        .zip(data.skip(1))
        .filter(|(a, b)| b > a)
        .count() as u32
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

        let data = r#"199
200
208
210
200
207
240
269
260
263"#;
        assert_eq!(solve(data), 7);
    }
}
