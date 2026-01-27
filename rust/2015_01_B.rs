use std::io;

fn main() {
    let mut input = String::new();
    let mut floor = 0;
    
    io::stdin().read_line(&mut input).expect("lineam legere non potui");
    for (pos, i) in input.chars().enumerate() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor < 0 {
            println!("{}", pos + 1);
            break;
        }
    }
}
