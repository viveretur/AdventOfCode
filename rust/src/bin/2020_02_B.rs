use std::io;

fn get_groups(line: &str) -> (u8, usize, usize, &[u8]) {
    let (t, pw) = line.split_once(": ").unwrap();
    let (range, ch) = t.split_once(' ').unwrap();
    let (min, max) = range.split_once('-').unwrap();

    (
        ch.bytes().next().unwrap(),
        min.parse::<usize>().unwrap(),
        max.parse::<usize>().unwrap(),
        pw.as_bytes(),
    )
}

fn solve(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let (ch, a, b, pw) = get_groups(line);
            (pw[a - 1] == ch) ^ (pw[b - 1] == ch)
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

        assert_eq!(solve(data), 1);
    }
}
