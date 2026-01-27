use std::io;

fn main() {
	let mut input = String::new();
	let mut d: i32 = 0;  // North
	let mut x = 0;
	let mut y = 0;

    io::stdin().read_line(&mut input).expect("lineam legere non potui");
    for key in input.trim().split(", ") {
    	let mut chars = key.chars();
    	let dir = chars.next().unwrap();
    	let amt: i32 = chars.as_str().parse().unwrap();
    	d = match dir {
    		'R' => (d + 1).rem_euclid(4),
    		'L' => (d - 1).rem_euclid(4),
    		_ => panic!("'{:?}'' debet esse L vel R", dir),
    	};
    	match d {
    		0 => y += amt,
    		1 => x += amt,
    		2 => y -= amt,
    		3 => x -= amt,
    		_ => panic!("directio invalida"),
    	}
    	println!("{key} => {d} : {amt}");
    }
    println!("{} + {} = {}", x.abs(), y.abs(), x.abs() + y.abs());
}
