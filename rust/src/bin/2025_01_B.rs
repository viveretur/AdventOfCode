use std::io;

use aoc::split_first_off;

fn step(pos: i32, delta: i32) -> (i32, u32) {
    let end = pos + delta;

    let clicks = if delta > 0 {
        (end / 100) as u32
    } else if delta < 0 {
        ((pos - 1).div_euclid(100) - (end - 1).div_euclid(100)) as u32
    } else {
        0
    };

    (end.rem_euclid(100), clicks)
}

fn solve(data: &str) -> u32 {
    let (combo, _pos) = data.lines().fold((0u32, 50i32), |(combo, pos), line| {
        let (dir, rest) = split_first_off(line);
        let amt: i32 = rest.parse().unwrap();

        let delta = match dir {
            'L' => -amt,
            'R' => amt,
            _ => panic!("recte non est!"),
        };

        let (next, clicks) = step(pos, delta);
        if cfg!(debug_assertions) {
            eprintln!("{pos} | {line} -> delta={delta} clicks={clicks} next={next}");
        }

        (combo + clicks, next)
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
L105
R260
L55
L1
L99
R14
L82"#;

        assert_eq!(solve(data), 9);
    }
}
