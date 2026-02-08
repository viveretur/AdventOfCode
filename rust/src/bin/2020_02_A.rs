use std::io;

fn get_groups(line: &str) -> (char, usize, usize, &str) {
    let (t, pw) = line.split_once(": ").unwrap();
    let (range, ch) = t.split_once(' ').unwrap();
    let (min, max) = range.split_once('-').unwrap();

    (
        ch.chars().next().unwrap(),
        min.parse().unwrap(),
        max.parse().unwrap(),
        pw,
    )
}

fn solve(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let (ch, min, max, pw) = get_groups(line);
            let count = pw.chars().filter(|&c| c == ch).count();
            (min..=max).contains(&count)
        })
        .count()
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
           1-3 a: abcde
           1-3 b: cdefg
           2-9 c: ccccccccc 
        "};

        assert_eq!(solve(data), 2);
    }
}
