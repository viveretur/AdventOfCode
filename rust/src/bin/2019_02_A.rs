use std::io;

fn build_machine(data: &str) -> Vec<usize> {
    data.trim()
        .split(',')
        .map(|piece| piece.parse::<usize>().unwrap())
        .collect()
}

fn solve(data: &str) -> usize {
    let mut inst = build_machine(data);
    let mut pos = 0;

    #[cfg(not(test))]
    {
        inst[1] = 12;
        inst[2] = 2;
    }
    loop {
        match inst[pos] {
            99 => return inst[0],
            1 => {
                let target = inst[pos + 3];
                let a = inst[pos + 1];
                let b = inst[pos + 2];
                inst[target] = inst[a] + inst[b];
            }
            2 => {
                let target = inst[pos + 3];
                let a = inst[pos + 1];
                let b = inst[pos + 2];
                inst[target] = inst[a] * inst[b];
            }
            _ => panic!("non est proper!"),
        }
        pos += 4;
    }
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        let data: Vec<(&str, usize)> = vec![
            ("1,9,10,3,2,3,11,0,99,30,40,50", 3500),
            ("1,0,0,0,99", 2),
            ("2,2,2,0,99", 4),
            ("2,4,4,0,99,0", 9801),
            ("1,1,1,4,99,5,6,0,99", 30),
        ];

        for (a, b) in data.into_iter() {
            assert_eq!(solve(a), b);
        }
    }
}
