use std::io;

fn is_safe_skip(nums: &[i32], skip: usize) -> bool {
    let mut sign = 0;
    let mut prev: Option<i32> = None;

    for (i, &n) in nums.iter().enumerate() {
        if i == skip {
            continue;
        }

        if let Some(p) = prev {
            let dif = n - p;
            if dif == 0 || dif.abs() > 3 {
                return false;
            }

            let s = dif.signum();
            if sign == 0 {
                sign = s
            }
            if s != sign {
                return false;
            }
        }
        prev = Some(n);
    }
    true
}

fn is_safe(line: &str) -> bool {
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (0..=nums.len()).any(|k| is_safe_skip(&nums, k))
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

        assert_eq!(solve(data), 4);
    }
}
