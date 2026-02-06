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

const NEXT: [[u8; 4]; 14] = [
    [0, 0, 0, 0], // Unused
    [1, 1, 1, 3], // L, U, R, D
    [2, 2, 3, 6],
    [2, 1, 4, 7],
    [3, 4, 4, 8],
    [5, 5, 6, 5],
    [5, 2, 7, 10],
    [6, 3, 8, 11],
    [7, 4, 9, 12],
    [8, 9, 9, 9],
    [10, 6, 11, 10],
    [10, 7, 12, 13],
    [11, 8, 12, 12],
    [13, 11, 13, 13],
];

fn solve(data: &str) -> String {
    let mut cur = 5u8;

    data.lines()
        .map(|line| {
            cur = line
                .bytes()
                .filter_map(|b| DIR_FROM_BYTE[b as usize])
                .fold(cur, |cur, dir| NEXT[cur as usize][dir as usize]);

            char::from_digit(cur as u32, 16).unwrap()
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
        assert_eq!(solve(data), "5db3");
    }
}
