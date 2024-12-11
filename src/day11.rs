
use std::fs;
use crate::solver::Solver;
use std::collections::HashMap;
pub struct Day11;
impl Solver<i64, i64> for Day11 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day11.txt").unwrap();
        let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
        let mut result: i64 = 0;
        for stone in file.split_whitespace() {
            let stone_num: i64 = stone.parse::<i64>().unwrap();
            result += Day11::solve(stone_num, &mut cache, 25);
        }
        return result;
    }

    fn solve_part2(&self) -> i64 {
        let file: String = fs::read_to_string("input/day11.txt").unwrap();
        let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
        let mut result: i64 = 0;
        for stone in file.split_whitespace() {
            let stone_num: i64 = stone.parse::<i64>().unwrap();
            result += Day11::solve(stone_num, &mut cache, 75);
        }
        return result;
    }
}

impl Day11 {
    fn solve(stone_num: i64, cache: &mut HashMap<(i64, i64), i64>, blinks: i64) -> i64 {
        if blinks == 0 {
            return 1i64;
        }
        if cache.contains_key(&(stone_num, blinks)) {
            return cache[&(stone_num, blinks)];
        }

        cache.insert((stone_num, blinks), 0);
        if stone_num == 0 {
            *cache.get_mut(&(stone_num, blinks)).unwrap() += Day11::solve(1, cache, blinks - 1);
        }
        else if Day11::even_digits(stone_num) {
            let (left, right) = Day11::split_digits(stone_num);
            *cache.get_mut(&(stone_num, blinks)).unwrap() += Day11::solve(left, cache, blinks - 1);
            *cache.get_mut(&(stone_num, blinks)).unwrap() += Day11::solve(right, cache, blinks - 1);
        }
        else {
            *cache.get_mut(&(stone_num, blinks)).unwrap() += Day11::solve(stone_num * 2024, cache, blinks - 1);
        }

        return cache[&(stone_num, blinks)];
    }

    fn even_digits(num: i64) -> bool {
        return (num.ilog10() + 1) % 2 == 0;
    }

    fn split_digits(num: i64) -> (i64, i64) {
        let divisor: i64 = 10i64.pow((num.ilog10() + 1) / 2);
        return (num / divisor, num % divisor);
    }
}
