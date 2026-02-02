use std::io::{self, Read};

fn solve(line: &str) -> i32 {
    let bytes = line.as_bytes();
    let mut total = 0i32;
    let mut last = *bytes.last().expect("non vacua");

    for &b in bytes {
        if b == last {
            total += (b - b'0') as i32;
        }
        last = b;
    }

    total
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

        assert_eq!(solve("1122"), 3);
        assert_eq!(solve("1111"), 4);
        assert_eq!(solve("1234"), 0);
        assert_eq!(solve("91212129"), 9);
    }
}
