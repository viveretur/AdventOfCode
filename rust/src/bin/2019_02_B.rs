use std::io;

fn build_machine(data: &str) -> Vec<usize> {
    data.trim()
        .split(',')
        .map(|piece| piece.parse::<usize>().unwrap())
        .collect()
}

fn run(machine: &Vec<usize>, noun: usize, verb: usize, mem: &mut [usize]) -> usize {
    mem.clone_from_slice(machine);
    mem[1] = noun;
    mem[2] = verb;

    let mut pos = 0;
    loop {
        match mem[pos] {
            99 => return mem[0],
            1 => {
                let (a, b, t) = (mem[pos + 1], mem[pos + 2], mem[pos + 3]);
                mem[t] = mem[a] + mem[b];
            }
            2 => {
                let (a, b, t) = (mem[pos + 1], mem[pos + 2], mem[pos + 3]);
                mem[t] = mem[a] * mem[b];
            }
            _ => panic!("non est proper!"),
        }
        pos += 4;
    }
}

fn solve(data: &str) -> usize {
    let machine = build_machine(data);
    let mut mem = machine.clone();

    for i in 0..100 {
        for j in 0..100 {
            if run(&machine, i, j, &mut mem) == 19690720 {
                return 100 * i + j;
            }
        }
    }
    panic!("non est proper");
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_machine, run};

    #[test]
    fn examples() {
        let data: Vec<(&str, usize, usize, usize)> = vec![
            ("1,9,10,3,2,3,11,0,99,30,40,50", 9, 10, 3500),
            ("1,0,0,0,99", 0, 0, 2),
            ("2,2,2,0,99", 2, 2, 4),
            ("2,4,4,0,99,0", 4, 4, 9801),
            ("1,1,1,4,99,5,6,0,99", 1, 1, 30),
        ];

        for (a, b, c, d) in data.into_iter() {
            let machine = build_machine(a);
            let mut mem = machine.clone();
            assert_eq!(run(&machine, b, c, &mut mem), d);
        }
    }
}
