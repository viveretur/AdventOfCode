use std::io;

trait FindMultipleExt: IntoIterator<Item = u32> + Sized {
    fn find_multiple(self) -> Option<u32> {
        let mut v: Vec<u32> = self.into_iter().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));

        for (i, &num) in v.iter().enumerate() {
            for &den in &v[i + 1..] {
                if num % den == 0 {
                    return Some(num / den);
                }
            }
        }
        None
    }
}
impl<I> FindMultipleExt for I where I: IntoIterator<Item = u32> {}

fn solve(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .find_multiple()
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
            5 9 2 8
            9 4 7 3
            3 8 6 5
        "};

        assert_eq!(solve(data), 9);
    }
}
