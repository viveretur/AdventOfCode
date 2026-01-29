use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    let line = input.trim().as_bytes();
    let half = line.len() / 2;
    let total: usize = (0..half)
        .filter(|&i| line[i] == line[i + half])
        .map(|i| (line[i] - b'0') as usize)
        .sum();
    println!("{}", 2 * total);
    Ok(())
}
