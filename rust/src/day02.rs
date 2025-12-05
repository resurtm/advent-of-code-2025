use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (u128, u128) {
    let (mut res0, res1) = (0, 0);
    fs::read_to_string(format!("../data/day02/{}.txt", test_name))
        .expect("cannot read input file")
        .split(",")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            let parts: Vec<&str> = x.split("-").collect();
            let begin: u128 = parts[0].parse().expect("i32 expected begin");
            let end: u128 = parts[1].parse().expect("i32 expected end");
            for i in begin..=end {
                let is = i.to_string();
                let il = is.len();
                if il % 2 == 0 && is.as_bytes()[..il / 2] == is.as_bytes()[il / 2..] {
                    res0 += i;
                }
            }
        });
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 01, part 1: {}", res0);
    println!("Day 01, part 2: {}", res1);
    (res0, res1)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
