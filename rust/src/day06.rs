use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (u128, u128) {
    let raw = fs::read_to_string(format!("../data/day06/{}.txt", test_name))
        .expect("cannot read input file");
    let lines: Vec<&str> = raw.split("\n").filter(|x| !x.is_empty()).collect();
    let res0 = solve_internal_part1(&lines);
    let res1 = solve_internal_part2(&lines);
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 06, part 1: {}", res0);
    println!("Day 06, part 2: {}", res1);
    (res0, res1)
}

fn solve_internal_part2(lines: &Vec<&str>) -> u128 {
    // read the widths and the ops of each column
    let mut widths = vec![];
    let mut width = 0;
    let mut ops = vec![];
    for ch in lines[lines.len() - 1].chars() {
        if ch == '+' || ch == '*' {
            widths.push(width);
            width = 0;
            ops.push(ch);
        } else {
            width += 1;
        }
    }
    widths.remove(0);
    widths.push(width + 1);

    // read the numbers
    let mut cols: Vec<Vec<Vec<char>>> = vec![vec![]; widths.len()];
    for line in lines.iter().take(lines.len() - 1) {
        let mut pos = 0;
        let mut cur = 0;
        while cur < widths.len() {
            let chunk: Vec<char> = line[pos..pos + widths[cur]].chars().collect();
            cols[cur].push(chunk);
            pos += widths[cur] + 1;
            cur += 1;
        }
    }

    // swap columns and rows
    for col in cols.iter_mut() {
        let mut new = vec![vec!['_'; col.len()]; col[0].len()];
        for i in 0..col.len() {
            for j in 0..col[i].len() {
                new[j][i] = col[i][j];
            }
        }
        *col = new;
    }

    let mut res = 0;
    for (idx, op) in ops.iter().enumerate() {
        let nums = cols[idx].iter().map(|its| {
            let s: String = its.iter().collect();
            s.trim().parse::<u128>().expect("u128 expected")
        });
        match op {
            '+' => {
                res += nums.sum::<u128>();
            }
            '*' => {
                res += nums.product::<u128>();
            }
            _ => panic!("invalid op"),
        }
    }
    res
}

fn solve_internal_part1(lines: &Vec<&str>) -> u128 {
    let data: Vec<Vec<&str>> = lines
        .iter()
        .map(|x| x.trim())
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
    let mut res = 0;
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
        res += val;
    }
    res
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
        assert_eq!(solve_internal("test0").1, 3_263_827);
    }
}
