use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (u128, u128) {
    let (mut res0, mut res1) = (0, 0);
    fs::read_to_string(format!("../data/day03/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            let chars: Vec<char> = x.chars().collect();
            // part 1
            let mut max = 0;
            for i in 0..chars.len() - 1 {
                for j in i + 1..chars.len() {
                    let val = format!("{}{}", chars[i], chars[j])
                        .parse::<u128>()
                        .expect("u128 expected");
                    if max < val {
                        max = val;
                    }
                }
            }
            res0 += max;
            // part 2
            let mut accum = vec![];
            let mut max = 0;
            check_joltage(0, 0, &mut accum, &mut max, &chars);
            res1 += max;
        });
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 03, part 1: {}", res0);
    println!("Day 03, part 2: {}", res1);
    (res0, res1)
}

fn check_joltage(
    pos: usize,
    level: usize,
    accum: &mut Vec<char>,
    max: &mut u128,
    chars: &Vec<char>,
) {
    if level == 12 {
        let val = accum
            .iter()
            .collect::<String>()
            .parse::<u128>()
            .expect("accum u128 expected");
        if *max < val {
            *max = val;
        }
        return;
    }
    let begin = if level == 0 { 0 } else { pos + 1 };
    for i in begin..chars.len() {
        accum.push(chars[i]);
        check_joltage(i, level + 1, accum, max, chars);
        accum.pop();
    }
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
        assert_eq!(solve_internal("test0").1, 3_121_910_778_619);
    }
}
