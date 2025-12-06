use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let parts: Vec<String> = fs::read_to_string(format!("../data/day05/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n\n")
        .map(String::from)
        .collect();
    let fresh: Vec<(u128, u128)> = parts[0]
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            let intv: Vec<u128> = x
                .split("-")
                .map(|y| y.parse::<u128>().expect("u128 fresh expected"))
                .collect();
            (intv[0], intv[1])
        })
        .collect();
    let avail: Vec<u128> = parts[1]
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u128>().expect("u128 avail expected"))
        .collect();
    let (mut res0, res1) = (0, 0);
    for &it in avail.iter() {
        for &(begin, end) in fresh.iter() {
            if begin <= it && it <= end {
                res0 += 1;
                break;
            }
        }
    }
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 05, part 1: {}", res0);
    println!("Day 05, part 2: {}", res1);
    (res0, res1)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
