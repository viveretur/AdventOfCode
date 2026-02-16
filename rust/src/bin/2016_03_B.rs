use std::io;

#[inline]
fn assert_triangle(a: usize, b: usize, c: usize) -> bool {
    let max = a.max(b).max(c);
    let sum = a + b + c;
    sum - max > max
}

fn solve(data: &str) -> usize {
    let nums: Vec<usize> = data
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    nums.chunks_exact(9)
        .map(|b| {
            assert_triangle(b[0], b[3], b[6]) as usize
                + assert_triangle(b[1], b[4], b[7]) as usize
                + assert_triangle(b[2], b[5], b[8]) as usize
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
           3  10  5
           4  20 13
           5  40 12
        "};

        assert_eq!(solve(data), 2);
    }
}
