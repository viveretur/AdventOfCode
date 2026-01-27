use std::io;

fn main() {
    let mut input = String::new();
    let mut floor = 0;
    
    io::stdin().read_line(&mut input).expect("lineam legere non potui");
    for i in input.chars() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("{}", floor);
}
