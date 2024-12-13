
use std::fs;
use crate::solver::Solver;
use regex::Regex;
pub struct Day13;
impl Solver<i64, i64> for Day13 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day13.txt").unwrap();
        let values: Vec<i64> = Regex::new(r"\d+").unwrap().find_iter(file.as_str()).into_iter()
                                                .map(|x| x.as_str().parse::<i64>().unwrap()).collect();

        let mut result: i64 = 0;

        let mut i: usize = 0;
        while i < values.len() {
            let ax: i64 = values[i];
            let ay: i64 = values[i + 1];
            let bx: i64 = values[i + 2];
            let by: i64 = values[i + 3];
            let px: i64 = values[i + 4];
            let py: i64 = values[i + 5];
            i += 6;

            result += Day13::get_solution(ax, ay, bx, by, px, py, Some(100));
        }
        return result;
    }

    fn solve_part2(&self) -> i64 {
        let file: String = fs::read_to_string("input/day13.txt").unwrap();
        const PADDING: i64 = 10000000000000;
        let values: Vec<i64> = Regex::new(r"\d+").unwrap().find_iter(file.as_str()).into_iter()
                                                .map(|x| x.as_str().parse::<i64>().unwrap()).collect();

        let mut result: i64 = 0;

        let mut i: usize = 0;
        while i < values.len() {
            let ax: i64 = values[i];
            let ay: i64 = values[i + 1];
            let bx: i64 = values[i + 2];
            let by: i64 = values[i + 3];
            let px: i64 = values[i + 4] + PADDING;
            let py: i64 = values[i + 5] + PADDING;
            i += 6;

            result += Day13::get_solution(ax, ay, bx, by, px, py, None);
        }
        return result;
    }
}


impl Day13 {
    fn get_solution(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64, limit: Option<i64>) -> i64 {
        let b_denominator = by * ax - bx * ay;
        let b_numerator = py * ax - px * ay;

        if b_numerator % b_denominator != 0 {
            return 0;
        }
        
        let b_presses = b_numerator / b_denominator;

        let a_numerator = px - b_presses * bx;

        if a_numerator % ax != 0 {
            return 0;
        }

        let a_presses = a_numerator / ax;
        
        if limit.is_some() && a_presses > limit.unwrap() && b_presses > limit.unwrap() {
            return 0;
        }
        return 3 * a_presses + b_presses;
    } 
}