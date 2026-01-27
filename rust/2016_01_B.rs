use std::collections::HashSet;
use std::io;

fn main() {
	let mut input = String::new();
	let mut d: i32 = 0;  // North
	let mut x = 0;
	let mut y = 0;
	let mut seen: HashSet<(i32, i32)> = HashSet::new();
	let mut loc = (0, 0);
	seen.insert(loc);

    io::stdin().read_line(&mut input).expect("lineam legere non potui");
    'find: for key in input.trim().split(", ") {
    	let mut chars = key.chars();
    	let dir = chars.next().unwrap();
    	let amt: i32 = chars.as_str().parse().unwrap();
    	d = match dir {
    		'R' => (d + 1).rem_euclid(4),
    		'L' => (d - 1).rem_euclid(4),
    		_ => panic!("'{:?}'' debet esse L vel R", dir),
    	};
		for _ in 1..=amt {
			match d {
  				0 => y += 1,
  				1 => x += 1,
  				2 => y -= 1,
  				3 => x -= 1,
    	        _ => println!("directio invalida"),
			}
			loc = (x, y);
			if seen.contains(&loc) {
				break 'find;
			} else {
				seen.insert(loc.clone());
			}
		}
    }
    println!("{} + {} = {}", x.abs(), y.abs(), x.abs() + y.abs());
}
