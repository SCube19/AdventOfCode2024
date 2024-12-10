use std::fs;
use crate::solver::Solver;

pub struct Day10;
impl Solver<u32, u32> for Day10 {
    fn solve_part1(&self) -> u32 {
        let file: String = fs::read_to_string("input/day10.txt").unwrap();
        let grid: Vec<Vec<u32>> = file.lines().into_iter().map(|line | line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        
        let mut result: u32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    result += Day10::dfs(&grid, i, j);
                }
            }
        }
        return result;
    }

    fn solve_part2(&self) -> u32 {
        let file: String = fs::read_to_string("input/day10.txt").unwrap();
        let grid: Vec<Vec<u32>> = file.lines().into_iter().map(|line | line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        let mut dp: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];
        
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    dp[i][j] = 1;
                }
            }
        }

        for i in 1..10 {
            Day10::fill_dp(&mut dp, &grid, i);
        }

        let mut result: u32 = 0;
        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                if grid[i][j] == 9 {
                    result += dp[i][j];
                }
            }
        }
        return result;
    }
}

impl Day10 {
    fn dfs(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut result: u32 = 0;

        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((i, j));

        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;

            if grid[i][j] == 9 {
                result += 1;
            }

            if grid.get(i - 1).unwrap_or(&vec![]).get(j).is_some_and(| cell | *cell == grid[i][j] + 1) {
                stack.push((i - 1, j));
            }
            if grid.get(i + 1).unwrap_or(&vec![]).get(j).is_some_and(| cell | *cell == grid[i][j] + 1) {
                stack.push((i + 1, j));
            }
            if grid.get(i).unwrap_or(&vec![]).get(j + 1).is_some_and(| cell | *cell == grid[i][j] + 1) {
                stack.push((i, j + 1));
            }
            if grid.get(i).unwrap_or(&vec![]).get(j - 1).is_some_and(| cell | *cell == grid[i][j] + 1) {
                stack.push((i, j - 1));
            }
        }

        return result;
    }

    fn fill_dp(dp: &mut Vec<Vec<u32>>, grid: &Vec<Vec<u32>>, height: u32) -> () {
        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                if grid[i][j] == height {
                    if *grid.get(i).unwrap_or(&vec![]).get(j - 1).unwrap_or(&10) == height - 1 {
                        dp[i][j] += dp[i][j - 1];
                    }
                    if *grid.get(i).unwrap_or(&vec![]).get(j + 1).unwrap_or(&10) == height - 1 {
                        dp[i][j] += dp[i][j + 1]
                    }
                    if *grid.get(i - 1).unwrap_or(&vec![]).get(j).unwrap_or(&10) == height - 1 {
                        dp[i][j] += dp[i - 1][j]
                    }
                    if *grid.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&10) == height - 1 {
                        dp[i][j] += dp[i + 1][j]
                    }
                }
            }
        }
    }
}
