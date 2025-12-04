use std::fs;

pub fn solve() {
    solve_internal("test0");
    // solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> i32 {
    let (_, res) = fs::read_to_string(format!("../data/day01/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            (
                x.as_bytes()[0],
                x[1..].parse::<i32>().expect("i32 expected"),
            )
        })
        .fold((50, 0), |mut acc, (dir, num)| {
            match dir {
                b'L' => acc.0 -= num % 100,
                b'R' => acc.0 += num % 100,
                _ => panic!("L or R expected"),
            }
            acc.0 %= 100;
            if acc.0 < 0 {
                acc.0 += 100
            }
            if acc.0 == 0 {
                acc.1 += 1;
            }
            acc
        });
    println!("Day 01, part 1: {}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0"), 3);
    }
}
