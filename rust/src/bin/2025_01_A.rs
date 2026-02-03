use std::io;

use aoc::split_first_off;

fn solve(data: &str) -> u32 {
    let (combo, _pos) = data.lines().fold((0u32, 50i32), |(combo, pos), line| {
        let (dir, rest) = split_first_off(line);
        let amt: i32 = rest.parse().unwrap();

        let delta = match dir {
            'L' => -amt,
            'R' => amt,
            _ => panic!("recte non est!"),
        };

        let pos = (pos + delta).rem_euclid(100);
        let combo = combo + u32::from(pos == 0);
        (combo, pos)
    });

    combo
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        let data = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(solve(data), 3);
    }
}
