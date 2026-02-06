use std::{char, io};

#[repr(u8)]
#[derive(Copy, Clone)]
enum Dir {
    L = 0,
    U = 1,
    R = 2,
    D = 3,
}

const DIR_FROM_BYTE: [Option<Dir>; 256] = {
    let mut t = [None; 256];
    t[b'L' as usize] = Some(Dir::L);
    t[b'U' as usize] = Some(Dir::U);
    t[b'R' as usize] = Some(Dir::R);
    t[b'D' as usize] = Some(Dir::D);
    t
};

const NEXT: [[u8; 4]; 10] = [
    [0, 0, 0, 0], // Unused
    [1, 1, 2, 4], // L, U, R, D
    [1, 2, 3, 5],
    [2, 3, 3, 6],
    [4, 1, 5, 7],
    [4, 2, 6, 8],
    [5, 3, 6, 9],
    [7, 4, 8, 7],
    [7, 5, 9, 8],
    [8, 6, 9, 9],
];

fn solve(data: &str) -> String {
    let mut cur = 5u8;

    data.lines()
        .map(|line| {
            cur = line
                .bytes()
                .filter_map(|b| DIR_FROM_BYTE[b as usize])
                .fold(cur, |cur, dir| NEXT[cur as usize][dir as usize]);

            char::from_digit(cur as u32, 10).unwrap()
        })
        .collect()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn examples() {
        let data = indoc! {"
            ULL
            RRDDD
            LURDL
            UUUD
        "};
        assert_eq!(solve(data), "1985");
    }
}
