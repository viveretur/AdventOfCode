use std::io;

fn find_ring(num: i32) -> i32 {
    (((num as f64).sqrt() - 1.0) / 2.0).ceil() as i32
}

fn find_offset(num: i32, ring: i32) -> i32 {
    // 1->1^2, 2->3^2, 3->5^2, 4->7^2, 5->9^2
    let prior_ring_end = (2 * ring - 1).pow(2);
    let ring_pos = num - prior_ring_end - 1;
    let side_pos = ring_pos % (2 * ring);
    // 2: 10->0->-1, 11->1->0,12->2->1,13->3->2
    // 3: 26->0->-2
    let offset = side_pos - ring + 1;
    #[cfg(debug_assertions)]
    println!(
        "{}, {} -> {}, {}, {}, {}",
        num, ring, prior_ring_end, ring_pos, side_pos, offset
    );
    offset.abs()
}

fn solve(data: &str) -> i32 {
    let num = data.trim().parse::<i32>().unwrap();
    // Corner case:
    if num == 1 {
        return 0;
    }
    let ring = find_ring(num);
    ring + find_offset(num, ring)
}

fn main() -> io::Result<()> {
    let data = io::read_to_string(io::stdin())?;
    println!("{}", solve(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("1"), 0);
        assert_eq!(solve("3"), 2);
        assert_eq!(solve("12"), 3);
        assert_eq!(solve("23"), 2);
        assert_eq!(solve("1024"), 31);
    }

    #[test]
    fn test_find_ring() {
        assert_eq!(find_ring(2), 1);
        assert_eq!(find_ring(9), 1);
        assert_eq!(find_ring(10), 2);
        assert_eq!(find_ring(25), 2);
        assert_eq!(find_ring(26), 3);
        assert_eq!(find_ring(289), 8);
        assert_eq!(find_ring(290), 9);
    }

    #[test]
    fn test_find_offset() {
        assert_eq!(find_offset(3, 1), 1);
        assert_eq!(find_offset(12, 2), 1);
        assert_eq!(find_offset(14, 2), 1);
        assert_eq!(find_offset(17, 2), 2);
    }
}
