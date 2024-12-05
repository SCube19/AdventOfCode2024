use std::fs;
use crate::solver::Solver;

pub struct Day2;
impl Solver<i32, i32> for Day2 {
    fn solve_part1(&self) -> i32 {
        let mut safe: i32 = 0;

        for line in fs::read_to_string("input/day2.txt").unwrap().lines() {
            let values: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if Day2::safety(&values) {
                safe += 1;
            }
        }
        return safe;
    }

    fn solve_part2(&self) -> i32 {
        let mut safe: i32 = 0;

        for line in fs::read_to_string("input/day2.txt").unwrap().lines() {
            let values: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if Day2::safety_forward(&values) || Day2::safety_backward(&values) {
                safe += 1;
            }
        }
        return safe;
    }
}

impl Day2 {
    fn check_indices(values: &Vec<i32>, i: usize, j: usize, increasing: bool) -> bool {
        if i >= values.len() || j >= values.len() {
            return true;
        }

        let diff: i32 = values.get(j).expect("err") - values.get(i).expect("err");
        let curr_increasing: bool = diff > 0;

        if increasing != curr_increasing {
            return false;
        }
        
        let diffabs: i32 = diff.abs();
        if diffabs < 1 || diffabs > 3 {
            return false;
        }
        return true;
    }

    fn safety(values: &Vec<i32>) -> bool {
        let increasing: bool = (values.get(1).unwrap() - values.get(0).unwrap()) > 0;

        let mut i: usize = 0;
        let mut j: usize = 1;
        while j < values.len() {
            if !Day2::check_indices(values, i, j, increasing) {
                return false;
            }

            i += 1;
            j += 1;
        }
        return true;
    }

    fn safety_forward(values: &Vec<i32>) -> bool {
        let increasing: bool = (values.get(1).unwrap() - values.get(0).unwrap()) > 0;

        let mut i: usize = 0;
        let mut j: usize = 1;
        let mut bad_flag: bool = false;

        while j < values.len() {
            if !Day2::check_indices(values, i, j, increasing) {
                if bad_flag || !Day2::check_indices(values, i, j + 1, increasing) {
                    return false;
                }
                bad_flag = true;
                i += 2;
                j += 2;
            }
            else {
                i += 1;
                j += 1;
            }

        }
        return true;
    }

    fn safety_backward(values: &Vec<i32>) -> bool {
        let increasing: bool = (values.get(values.len() - 2).unwrap() - values.get(values.len() - 1).unwrap()) > 0;

        let mut i: usize = values.len() as usize - 1;
        let mut j: usize = values.len() as usize - 2;
        let mut bad_flag: bool = false;

        loop {
            if !Day2::check_indices(values, i, j, increasing) {
                if bad_flag || !Day2::check_indices(values, i, j.checked_sub(1).unwrap_or(values.len()), increasing) {
                    return false;
                }
                bad_flag = true;
                i = match i.checked_sub(2) {
                    Some(x) => x,
                    None => return true,
                };
                j = match j.checked_sub(2) {
                    Some(x) => x,
                    None => return true,
                };
            }
            else {
                i = match i.checked_sub(1) {
                    Some(x) => x,
                    None => return true,
                };
                j = match j.checked_sub(1) {
                    Some(x) => x,
                    None => return true,
                };
            }

        }
    }
}