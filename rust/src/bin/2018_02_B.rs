use std::io;

fn quantify_difference(me: &str, other: &str) -> usize {
    let diff = me.len().abs_diff(other.len());
    diff + me
        .bytes()
        .zip(other.bytes())
        .map(|(a, b)| (a != b) as usize)
        .sum::<usize>()
}

fn remove_common(me: &str, other: &str) -> String {
    me.bytes()
        .zip(other.bytes())
        .filter_map(|(a, b)| (a == b).then_some(a as char))
        .collect()
}

fn solve(data: &str) -> String {
    let lines: Vec<&str> = data.lines().collect();
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if quantify_difference(lines[i], lines[j]) == 1 {
                return remove_common(lines[i], lines[j]);
            }
        }
    }
    panic!("non est");
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
			abcde
			fghij
			klmno
			pqrst
			fguij
			axcye
			wvxzy
		"};

        assert_eq!(solve(data), "fgij");
    }
}
