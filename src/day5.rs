use std::fs;
use std::collections::{HashMap, HashSet};
use crate::solver::Solver;

pub struct Day5;
impl Solver<i32, i32> for Day5 {
    fn solve_part1(&self) -> i32 {
        let ordering_file = fs::read_to_string("input/day5atest.txt");
        let print_file = fs::read_to_string("input/day5btest.txt");

        let mut ordering: HashMap<i32, HashSet<i32>> = HashMap::new();
        for line in ordering_file.unwrap().lines() {
            let values = line.split("|").into_iter().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            if ordering.get(values.get(0).unwrap()).is_none() {
                ordering.insert(*values.get(0).unwrap(), HashSet::new());
            }
            ordering.get_mut(values.get(0).unwrap()).unwrap().insert(*values.get(1).unwrap());
        }

        let mut result: i32 = 0;
        for line in print_file.unwrap().lines() {
            let print: Vec<i32> = line.split(",").into_iter().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            result += 1;
            for i in 0..print.len() {
                if Day5::breaks_ordering(&ordering, &print, i) {
                    result -= 1;
                    break;
                }
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i32 {
        return 0;
    }
}

impl Day5 {
    fn breaks_ordering(ordering: &HashMap<i32, HashSet<i32>>, print: &Vec<i32>, page: usize) -> bool {
        let set: Option<&HashSet<i32>> = ordering.get(print.get(page).unwrap());
        println!("{:?}", set);
        if set.is_none() {
            return false;
        }

        for i in 0..page {
            if set.unwrap().contains(print.get(i).unwrap()) {
                return true;
            }
        }
        return false;
    }
}
