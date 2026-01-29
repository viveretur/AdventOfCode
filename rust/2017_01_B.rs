use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut total = 0;

    io::stdin()
        .read_line(&mut input)
        .expect("lineam legere non potui");
    let line: Vec<char> = input.trim().chars().collect();
    let len = line.len();
    for i in 0..len {
        if line.get(i) == line.get((i + len / 2) % len) {
            total += line
                .get(i)
                .expect("Need to improve this")
                .to_digit(10)
                .unwrap();
        }
    }
    println!("{total}");
    Ok(())
}
