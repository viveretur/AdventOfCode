use std::collections::HashMap;
use std::io;

trait CharCountsExt {
    fn char_counts(&self) -> HashMap<char, usize>;
}

impl CharCountsExt for str {
    fn char_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for ch in self.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        counts
    }
}

fn has_2_and_3(counts: &HashMap<char, usize>) -> (u32, u32) {
    let mut has2 = false;
    let mut has3 = false;

    for &c in counts.values() {
        has2 |= c == 2;
        has3 |= c == 3;
        if has2 && has3 {
            break;
        }
    }

    (has2 as u32, has3 as u32)
}

fn solve(data: &str) -> u32 {
    let (twos, threes) = data.lines().fold((0u32, 0u32), |(t2, t3), line| {
        let counts = line.char_counts();
        let (a, b) = has_2_and_3(&counts);
        (t2 + a, t3 + b)
    });
    twos * threes
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
			abcdef
			bababc
			abbcde
			abcccd
			aabcdd
			abcdee
			ababab
		"};

        assert_eq!(solve(data), 12);
    }
}
