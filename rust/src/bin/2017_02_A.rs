use itertools::Itertools;
use std::io;

fn solve(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let nums = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            let (min, max) = nums.minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn examples() {
        let data = indoc! {"
            5 1 9 5
            7 5 3
            2 4 6 8
        "};

        assert_eq!(solve(data), 18);
    }
}
