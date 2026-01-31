use std::io::{self, Read};

fn solve(line: &str) -> u32 {
    let bytes = line.as_bytes();
    let half = bytes.len() / 2;

    // trick: only used half array, rotationally symmetric, so double total.
    let half_sum: u32 = (0..half)
        .filter(|&i| bytes[i] == bytes[i + half])
        .map(|i| (bytes[i] - b'0') as u32)
        .sum();

    half_sum * 2
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line)?;

    let total = solve(&line.trim());
    println!("{total}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        assert_eq!(solve("1212"), 6);
        assert_eq!(solve("1221"), 0);
        assert_eq!(solve("123425"), 4);
        assert_eq!(solve("123123"), 12);
        assert_eq!(solve("12131415"), 4);
    }
}
