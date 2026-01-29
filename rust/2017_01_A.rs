use std::io;

fn main() {
    let mut input = String::new();
    let mut total = 0;
    let mut last = 'X';

    io::stdin()
        .read_line(&mut input)
        .expect("lineam legere non potui");
    input = input.trim().to_string();
    let first = input.chars().next().unwrap();
    for i in input.chars() {
        if i == last {
            total += i.to_digit(10).unwrap();
        }
        last = i;
    }
    println!("{first}, {last}");
    if first == last {
        total += last.to_digit(10).unwrap();
    }
    println!("{total}");
}
