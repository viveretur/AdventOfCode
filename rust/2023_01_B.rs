use std::io;

enum Dir { Left, Right }

struct NumberMap {
    name: &'static [u8],
    value: u32,
}

const NUMBER_MAP: &[NumberMap] = &[
    NumberMap { name: b"zero",  value: 0 },
    NumberMap { name: b"one",   value: 1 },
    NumberMap { name: b"two",   value: 2 },
    NumberMap { name: b"three", value: 3 },
    NumberMap { name: b"four",  value: 4 },
    NumberMap { name: b"five",  value: 5 },
    NumberMap { name: b"six",   value: 6 },
    NumberMap { name: b"seven", value: 7 },
    NumberMap { name: b"eight", value: 8 },
    NumberMap { name: b"nine",  value: 9 },
];

fn find_number(line: &str, dir: Dir) -> u32 {
    let bytes = line.as_bytes();
    let len = bytes.len();

    for k in 0..len {
        let idx = match dir {
            Dir::Left => k,
            Dir::Right => len - k - 1,
        };
        let b = bytes[idx];

        if b.is_ascii_digit() {
            return (b - b'0') as u32;
        }

        if let Some(n) = NUMBER_MAP
            .iter()
            .find_map(|nm| bytes[idx..].starts_with(nm.name).then_some(nm.value))
        {
            return n;
        }
    };

    panic!("nullus numerus inventus est");
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            find_number(line, Dir::Left) * 10 + find_number(line, Dir::Right)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("{}", solve(&input));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(solve(input), 281);
    }
}
