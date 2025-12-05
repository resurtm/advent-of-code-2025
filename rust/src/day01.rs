use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let (_, res0, res1) = fs::read_to_string(format!("../data/day01/{}.txt", test_name))
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
        .fold((50, 0, 0), |mut acc, (dir, num)| {
            match dir {
                b'L' => {
                    for _ in 0..num {
                        acc.0 -= 1;
                        if acc.0 == -1 {
                            acc.0 += 100;
                        }
                        if acc.0 == 0 {
                            acc.2 += 1;
                        }
                    }
                }
                b'R' => {
                    for _ in 0..num {
                        acc.0 += 1;
                        if acc.0 == 100 {
                            acc.0 -= 100;
                        }
                        if acc.0 == 0 {
                            acc.2 += 1;
                        }
                    }
                }
                _ => panic!("L or R expected"),
            }
            if acc.0 == 0 {
                acc.1 += 1;
            }
            acc
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
        assert_eq!(solve_internal("test0").0, 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 6);
    }
}
