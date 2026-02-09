use std::io;

fn solve(data: &str) -> u32 {
    let (mut h, mut d) = (0u32, 0u32);

    for (cmd, n) in data.lines().map(|line| line.split_once(' ').unwrap()) {
        let val: u32 = n.parse().unwrap();
        match cmd {
            "forward" => h += val,
            "up" => d -= val,
            "down" => d += val,
            _ => panic!("non est proper!"),
        }
    }
    h * d
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
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
        "};

        assert_eq!(solve(data), 150);
    }
}
