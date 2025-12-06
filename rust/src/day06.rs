use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (u128, u128) {
    let raw = fs::read_to_string(format!("../data/day06/{}.txt", test_name))
        .expect("cannot read input file");
    let data: Vec<Vec<&str>> = raw
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .collect();
    let mut nums: Vec<Vec<u128>> = vec![];
    let mut ops: Vec<char> = vec![];
    for (idx, line) in data.iter().enumerate() {
        if idx == data.len() - 1 {
            ops = line.iter().map(|x| x.as_bytes()[0] as char).collect();
        } else {
            nums.push(
                line.iter()
                    .map(|x| x.parse::<u128>().expect("u128 expected"))
                    .collect(),
            );
        }
    }
    let (mut res0, res1) = (0, 0);
    for (i, op) in ops.iter().enumerate() {
        let mut val = 0;
        match op {
            '+' => {
                for num in nums.iter() {
                    val += num[i];
                }
            }
            '*' => {
                val = 1;
                for num in nums.iter() {
                    val *= num[i];
                }
            }
            _ => panic!("invalid op"),
        }
        res0 += val;
    }
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 06, part 1: {}", res0);
    println!("Day 06, part 2: {}", res1);
    (res0, res1)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 4_277_556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
