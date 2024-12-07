
use std::fs;
use crate::solver::Solver;

pub struct Day7;
impl Solver<i64, i64> for Day7 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day7.txt").unwrap();

        let mut result: i64 = 0;
        for line in file.lines() {
            let target: i64 = line[0..line.find(":").unwrap()].parse::<i64>().unwrap();
            let values: Vec<i64> = line[(line.find(":").unwrap() + 1)..].split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

            if Day7::try_to_match(&values, target) {
                result += target;
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i64 {
        let file: String = fs::read_to_string("input/day7.txt").unwrap();

        let mut result: i64 = 0;
        for line in file.lines() {
            let target: i64 = line[0..line.find(":").unwrap()].parse::<i64>().unwrap();
            let values: Vec<i64> = line[(line.find(":").unwrap() + 1)..].split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

            if Day7::try_to_match_extended(&values, target) {
                result += target;
            }
        }
        return result;
    }
}


impl Day7 {
    fn match_helper(values: &Vec<i64>, target: i64, i: usize, curr_value: i64) -> bool {
        if curr_value == target && i == values.len() {
            return true;
        }
        if i == values.len() || curr_value > target {
            return false;
        }

        return Day7::match_helper(values, target, i + 1, curr_value + values[i]) || Day7::match_helper(values, target, i + 1, curr_value * values[i]);
    }

    fn try_to_match(values: &Vec<i64>, target: i64) -> bool {
        return Day7::match_helper(values, target, 2, values[0] * values[1]) || Day7::match_helper(values, target, 2, values[0] + values[1]);
    }

    fn concat(a: i64, b: i64) -> i64 {
        return (a.to_string() + &b.to_string()).parse::<i64>().unwrap();
    }

    fn match_helper_extended(values: &Vec<i64>, target: i64, i: usize, curr_value: i64) -> bool {
        if curr_value == target && i == values.len() {
            return true;
        }
        if i == values.len() || curr_value > target {
            return false;
        }

        return Day7::match_helper_extended(values, target, i + 1, curr_value + values[i]) || 
               Day7::match_helper_extended(values, target, i + 1, curr_value * values[i]) ||
               Day7::match_helper_extended(values, target, i + 1, Day7::concat(curr_value,values[i]));
    }

    fn try_to_match_extended(values: &Vec<i64>, target: i64) -> bool {
        return Day7::match_helper_extended(values, target, 2, values[0] * values[1]) || 
               Day7::match_helper_extended(values, target, 2, values[0] + values[1]) ||
               Day7::match_helper_extended(values, target, 2, Day7::concat(values[0], values[1]));
    }
}