use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParseClaimError;

impl fmt::Display for ParseClaimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "non est proper")
    }
}

impl Error for ParseClaimError {}

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "#123 @ 3,2: 5x4"
        let s = s.trim();
        let (id_part, rest) = s.split_once(" @ ").ok_or(ParseClaimError)?;
        let id = id_part
            .strip_prefix('#')
            .ok_or(ParseClaimError)?
            .parse()
            .map_err(|_| ParseClaimError)?;

        let (pos, size) = rest.split_once(": ").ok_or(ParseClaimError)?;
        let (x, y) = pos.split_once(',').ok_or(ParseClaimError)?;
        let (w, h) = size.split_once('x').ok_or(ParseClaimError)?;

        Ok(Claim {
            id,
            x: x.parse().map_err(|_| ParseClaimError)?,
            y: y.parse().map_err(|_| ParseClaimError)?,
            width: w.parse().map_err(|_| ParseClaimError)?,
            height: h.parse().map_err(|_| ParseClaimError)?,
        })
    }
}

impl Claim {
    fn get_cells(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        let xs = self.x..self.x + self.width;
        let ys = self.y..self.y + self.height;

        xs.flat_map(move |x| ys.clone().map(move |y| (x, y)))
    }
}

fn solve(data: &str) -> Result<u32, ParseClaimError> {
    let claims: Vec<Claim> = data.lines().map(str::parse).collect::<Result<_, _>>()?;

    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in &claims {
        for cell in claim.get_cells() {
            *map.entry(cell).or_default() += 1;
        }
    }

    let id = claims
        .iter()
        .find_map(|c| {
            (c.get_cells()
                .all(|i| map.get(&i).copied().unwrap_or(0) == 1))
            .then_some(c.id)
        })
        .unwrap();

    Ok(id)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn examples() {
        let data = indoc! {"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 4x4
            #3 @ 5,5: 2x2
        "};

        assert_eq!(solve(&data).unwrap(), 3);
    }

    #[test]
    fn test_parse_claim() {
        let line = "#123 @ 3,2: 5x4";
        let expected = Ok(Claim {
            id: 123,
            x: 3,
            y: 2,
            width: 5,
            height: 4,
        });

        assert_eq!(line.parse(), expected);
    }

    #[test]
    fn test_claim_get_inches() {
        let claim = Claim {
            id: 123,
            x: 5,
            y: 4,
            width: 3,
            height: 2,
        };
        #[rustfmt::skip]  // (x, y)
        let expected = vec![
            (5, 4), (5, 5),
            (6, 4), (6, 5),
            (7, 4), (7, 5),
        ];

        assert_eq!(claim.get_cells().collect::<Vec<_>>(), expected);
    }
}
