use std::io::{self, Read};

fn fuel_requirements(mass: i32) -> i32 {
    mass / 3 - 2
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

        assert_eq!(solve("12"), 2);
        assert_eq!(solve("14"), 2);
        assert_eq!(solve("1969"), 654);
        assert_eq!(solve("100756"), 33583);
    }
}
