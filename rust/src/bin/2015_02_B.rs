use std::io;

struct Parcel {
    x: u64,
    y: u64,
    z: u64,
}

impl Parcel {
    fn new(line: &str) -> Self {
        let mut b = line.split('x').map(|i| i.parse::<u64>().unwrap());
        Self {
            x: b.next().unwrap(),
            y: b.next().unwrap(),
            z: b.next().unwrap(),
        }
    }

    fn get_ribbon(&self) -> u64 {
        let xy = self.x + self.y;
        let xz = self.x + self.z;
        let yz = self.y + self.z;

        self.x * self.y * self.z + 2 * [xy, xz, yz].into_iter().min().unwrap()
    }
}

fn solve(data: &str) -> u64 {
    data.lines()
        .map(|line| Parcel::new(line).get_ribbon())
        .sum()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {
        use super::solve;

        assert_eq!(solve("2x3x4"), 34);
        assert_eq!(solve("1x1x10"), 14);
    }
}
