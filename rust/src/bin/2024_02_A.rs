use itertools::Itertools;
use std::io;

fn is_safe(line: &str) -> bool {
    let conds = line
        .split(' ')
        .tuple_windows()
        .map(|(a, b)| b.parse::<i32>().unwrap() - a.parse::<i32>().unwrap());
    conds.clone().all(|a| a >= 1 && a <= 3) || conds.clone().all(|a| a >= -3 && a <= -1)
}

fn solve(data: &str) -> usize {
    data.lines().filter(|line| is_safe(line)).count()
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
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        assert_eq!(solve(data), 2);
    }
}
