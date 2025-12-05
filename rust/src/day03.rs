use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let (mut res0, res1) = (0, 0);
    fs::read_to_string(format!("../data/day03/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            let chars: Vec<char> = x.chars().collect();
            let mut max = 0;
            for i in 0..chars.len() - 1 {
                for j in i + 1..chars.len() {
                    let val = format!("{}{}", chars[i], chars[j])
                        .parse::<i32>()
                        .expect("i32 expected");
                    if max < val {
                        max = val;
                    }
                }
            }
            res0 += max;
        });
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 03, part 1: {}", res0);
    println!("Day 03, part 2: {}", res1);
    (res0, res1)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
