use std::io;

#[derive(Clone, Copy)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    fn idx(self) -> usize {
        match self {
            Colour::Red => 0,
            Colour::Green => 1,
            Colour::Blue => 2,
        }
    }

    fn parse(s: &str) -> Colour {
        match s {
            "red" => Colour::Red,
            "green" => Colour::Green,
            "blue" => Colour::Blue,
            _ => panic!("non est proper!"),
        }
    }
}

#[derive(Clone, Copy)]
struct Game {
    id: usize,
    max: [usize; 3],
}

impl Game {
    fn power(&self) -> usize {
        self.max[0] * self.max[1] * self.max[2]
    }
}

fn parse_line(line: &str) -> Game {
    let (game, plays) = line.split_once(": ").unwrap();
    let (_, id) = game.split_once(' ').unwrap();

    let mut max = [0usize; 3];

    for play in plays.split("; ") {
        for item in play.split(", ") {
            let (count, colour) = item.split_once(' ').unwrap();
            let n: usize = count.parse().unwrap();
            let c = Colour::parse(colour).idx();
            max[c] = max[c].max(n);
        }
    }
    Game {
        id: id.parse().unwrap(),
        max,
    }
}

fn solve(data: &str) -> usize {
    data.lines().map(parse_line).map(|g| g.power()).sum()
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn examples() {
        let data = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(solve(data), 2286);
    }

    #[test]
    fn test_parser() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result = parse_line(&line);
        assert_eq!(result.id, 1);
        assert_eq!(result.max[0], 4);
        assert_eq!(result.max[1], 2);
        assert_eq!(result.max[2], 6);
    }
}
