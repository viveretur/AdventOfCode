use std::io;

// rows: them (A=Rock, B=Paper, C=Scissors)
// cols: you  (X=Lose, Y=Draw, C=Win)
const MATCHUP_POINTS: [[u32; 3]; 3] = [
    [3, 4, 8], // R-S, R-R, R-P
    [1, 5, 9], // P-R, P-P, P-S
    [2, 6, 7], // S-P, S-S, S-R
];

fn score(them: u8, you: u8) -> u32 {
    debug_assert!((b'A'..=b'C').contains(&them));
    debug_assert!((b'X'..=b'Z').contains(&you));

    MATCHUP_POINTS[(them - b'A') as usize][(you - b'X') as usize]
}

fn solve(data: &str) -> u32 {
    data.lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let bytes = line.as_bytes();
            score(bytes[0], bytes[2])
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
			A Y
			B X
			C Z
		"};

        assert_eq!(solve(data), 12)
    }
}
