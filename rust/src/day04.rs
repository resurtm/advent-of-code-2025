use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let grid: Vec<Vec<bool>> = fs::read_to_string(format!("../data/day04/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| y == '@').collect())
        .fold(vec![], |mut acc, x| {
            acc.push(x);
            acc
        });
    let res0 = solve_internal_part1(&grid);
    let res1 = 0;
    println!("--- --- --- --- ---");
    println!("Test name: {}", test_name);
    println!("Day 04, part 1: {}", res0);
    println!("Day 04, part 2: {}", res1);
    (res0, res1)
}

fn solve_internal_part1(grid: &[Vec<bool>]) -> i32 {
    let (w, h) = (grid[0].len(), grid.len());
    let mut res = 0;
    for j in 0..h as i32 {
        for i in 0..w as i32 {
            if !grid[j as usize][i as usize] {
                continue;
            }
            if [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
                (i, j - 1),
                (i, j + 1),
            ]
            .into_iter()
            .filter(|&(x, y)| x >= 0 && x < w as i32 && y >= 0 && y < h as i32)
            .fold(0, |acc, (x, y)| {
                if grid[y as usize][x as usize] {
                    acc + 1
                } else {
                    acc
                }
            }) < 4
            {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
