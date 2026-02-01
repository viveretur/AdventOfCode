use std::cmp::Ordering;
use std::io;

fn solve(input: &str) -> u64 {
    let mut entries = input
        .lines()
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    entries.sort_unstable();

    for k in (2..entries.len()).rev() {
        let (mut i, mut j) = (0usize, k - 1);

        while i < j {
            let sum = entries[i] + entries[j] + entries[k];
            match sum.cmp(&2020) {
                Ordering::Equal => return entries[i] * entries[j] * entries[k],
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
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

        assert_eq!(solve("1721\n979\n366\n299\n675\n1456"), 241861950);
    }
}
