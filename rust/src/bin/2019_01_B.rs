use std::io::{self, Read};
use std::iter::successors;

fn fuel_requirements(mass: i32) -> i32 {
    successors(Some(mass), |&m| {
        let next = m / 3 - 2;
        (next > 0).then_some(next)
    })
    .skip(1)
    .sum()
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|m| fuel_requirements(m.parse::<i32>().unwrap()))
        .sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = solve(&input);
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        assert_eq!(solve("14"), 2);
        assert_eq!(solve("1969"), 966);
        assert_eq!(solve("100756"), 50346);
    }
}
