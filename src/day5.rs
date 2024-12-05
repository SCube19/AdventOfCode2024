use std::fs;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use crate::solver::Solver;

pub struct Day5;
impl Solver<i32, i32> for Day5 {
    fn solve_part1(&self) -> i32 {
        let ordering_file = fs::read_to_string("input/day5a.txt");
        let print_file = fs::read_to_string("input/day5b.txt");

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
            if !Day5::breaks_ordering(&print, &ordering) {
                result += print.get(print.len() / 2).unwrap();
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i32 {
        let ordering_file = fs::read_to_string("input/day5a.txt");
        let print_file = fs::read_to_string("input/day5b.txt");

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
            let mut print: Vec<i32> = line.split(",").into_iter().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            if Day5::breaks_ordering(&print, &ordering) {
                Day5::toposort(&mut print, &ordering);
                result += print.get(print.len() / 2).unwrap();
            }
        }
        return result;
    }
}

impl Day5 {
    fn toposort(values: &mut Vec<i32>, ordering: &HashMap<i32, HashSet<i32>>) -> () {
        values.sort_by(|a, b| {
            if !ordering.contains_key(a) && !ordering.contains_key(b) {
                return Ordering::Equal;
            }
            if ordering.get(a).unwrap_or(&HashSet::new()).contains(b) {
                return Ordering::Less;
            }
            if ordering.get(b).unwrap_or(&HashSet::new()).contains(a) {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        });
    }

    fn breaks_ordering(print: &Vec<i32>, ordering: &HashMap<i32, HashSet<i32>>) -> bool {
        let mut sorted = print.clone();
        Day5::toposort(&mut sorted, ordering);
        return sorted.cmp(print) != Ordering::Equal;
    }
}
