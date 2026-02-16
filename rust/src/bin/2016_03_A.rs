use std::io;

fn assert_triangle(line: &str) -> bool {
    let mut nums: Vec<usize> = line
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    debug_assert!(nums.len() == 3);
    nums.sort();
    nums[0] + nums[1] > nums[2]
}

fn solve(data: &str) -> usize {
    data.lines().filter(|line| assert_triangle(line)).count()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("5 10 25"), 0);
        assert_eq!(solve("3 4 5"), 1);
    }
}
