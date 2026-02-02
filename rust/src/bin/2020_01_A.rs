use std::cmp::Ordering;
use std::io;

fn solve(input: &str) -> u32 {
    let mut entries = input
        .lines()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    entries.sort_unstable();

    let (mut i, mut j) = (0usize, entries.len() - 1);

    while i < j {
        let sum = entries[i] + entries[j];
        match sum.cmp(&2020) {
            Ordering::Equal => return entries[i] * entries[j],
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }
    unreachable!()
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

        assert_eq!(solve("1721\n979\n366\n299\n675\n1456"), 514579);
        assert_eq!(solve("1731\n1020\n366\n299\n1000\n1456"), 1020000);
    }
}
