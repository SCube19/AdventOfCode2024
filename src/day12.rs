
use std::{collections::VecDeque, fs, collections::HashSet};
use crate::solver::Solver;

pub struct Day12;
impl Solver<i64, i64> for Day12 {
    fn solve_part1(&self) -> i64 {
        let file: String = fs::read_to_string("input/day12.txt").unwrap();
        let grid: Vec<Vec<char>> = file.lines().map(|x|x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        
        let mut result: i64 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited[i][j] {
                    result += Day12::make_plot(&grid, &mut visited, i, j, grid[i][j]);
                }
            }
        }
        return result;
    }

    fn solve_part2(&self) -> i64 {
        let file: String = fs::read_to_string("input/day12.txt").unwrap();
        let grid: Vec<Vec<char>> = file.lines().map(|x|x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        
        let mut result: i64 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if !visited[i][j] {
                    result += Day12::make_plot_sides(&grid, &mut visited, i, j, grid[i][j]);
                }
            }
        }
        return result;
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Day12 {
    fn make_plot(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, plant: char) -> i64 {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((i, j));

        let mut area: i64 = 0;
        let mut perimeter: i64 = 0;
        
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            if visited[i][j] || grid[i][j] != plant {
                continue;
            }
            visited[i][j] = true;
            
            area += 1;
            perimeter += (*grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j).unwrap_or(&'?') != plant) as i64 + 
                         (*grid.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&'?') != plant) as i64 + 
                         (*grid.get(i).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&'?') != plant) as i64 +
                         (*grid.get(i).unwrap_or(&vec![]).get(j + 1).unwrap_or(&'?') != plant) as i64;

            if grid.get(i.overflowing_sub(1).0).is_some_and(|x|x.get(j).is_some()) {
                q.push_back((i - 1, j));
            }
            if grid.get(i + 1).is_some_and(|x|x.get(j).is_some()) {
                q.push_back((i + 1, j));
            }
            if grid.get(i).is_some_and(|x|x.get(j.overflowing_sub(1).0).is_some()) {
                q.push_back((i, j - 1));
            }
            if grid.get(i).is_some_and(|x|x.get(j + 1).is_some()) {
                q.push_back((i, j + 1));
            }
        }

        return area * perimeter;
    }   

    fn make_plot_sides(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, plant: char) -> i64 {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((i, j));

        let mut area: i64 = 0;


        let mut borders: HashSet<(usize, usize, DIR)> = HashSet::new();
        
        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();
            if visited[i][j] || grid[i][j] != plant {
                continue;
            }
            visited[i][j] = true;
            
            area += 1;

            if *grid.get(i.overflowing_sub(1).0).unwrap_or(&vec![]).get(j).unwrap_or(&'?') == plant {
                q.push_back((i - 1, j));
            }
            else {
                borders.insert((i, j, DIR::RIGHT));
            }

            if *grid.get(i + 1).unwrap_or(&vec![]).get(j).unwrap_or(&'?') == plant {
                q.push_back((i + 1, j));
            }
            else {
                borders.insert((i, j, DIR::LEFT));
            }

            if *grid.get(i).unwrap_or(&vec![]).get(j.overflowing_sub(1).0).unwrap_or(&'?') == plant {
                q.push_back((i, j - 1));
            }
            else {
                borders.insert((i, j, DIR::UP));
            }

            if *grid.get(i).unwrap_or(&vec![]).get(j + 1).unwrap_or(&'?') == plant {
                q.push_back((i, j + 1));
            }
            else {
                borders.insert((i, j, DIR::DOWN));
            }
        }

        let mut visitedBorders: HashSet<(usize, usize, DIR)> = HashSet::new();
        let mut sides: i64 = 0;

        for (i, j, dir) in borders.iter() {
            if !visitedBorders.contains(&(*i, *j, dir.clone())) {
                Day12::visit_side(&borders, &mut visitedBorders, *i, *j, dir);
                sides += 1; 
            }
        }
        return area * sides;
    }   

    fn visit_side(borders: &HashSet<(usize, usize, DIR)>, visited: &mut HashSet<(usize, usize, DIR)>, i: usize, j: usize, dir: &DIR) {
        let mut q: VecDeque<(usize, usize, DIR)> = VecDeque::new();
        q.push_back((i, j, dir.clone()));  

        while !q.is_empty() {
            let (i, j, dir) = q.pop_front().unwrap();
            if visited.contains(&(i, j, dir.clone())) {
                continue;
            }
            visited.insert((i, j, dir.clone()));

            match dir.clone() {
                DIR::UP => {
                    if borders.contains(&(i.overflowing_sub(1).0, j, DIR::UP)) {
                        q.push_back((i.overflowing_sub(1).0, j, DIR::UP));
                    }
                    if borders.contains(&(i + 1, j, DIR::UP)) {
                        q.push_back((i + 1, j, DIR::UP));
                    }
                },
                DIR::DOWN => {
                    if borders.contains(&(i.overflowing_sub(1).0, j, DIR::DOWN)) {
                        q.push_back((i.overflowing_sub(1).0, j, DIR::DOWN));
                    }
                    if borders.contains(&(i + 1, j, DIR::DOWN)) {
                        q.push_back((i + 1, j, DIR::DOWN));
                    }
                },
                DIR::LEFT => {
                    if borders.contains(&(i, j.overflowing_sub(1).0, DIR::LEFT)) {
                        q.push_back((i, j.overflowing_sub(1).0, DIR::LEFT));
                    }
                    if borders.contains(&(i, j + 1, DIR::LEFT)) {
                        q.push_back((i, j + 1, DIR::LEFT));
                    }
                },
                DIR::RIGHT => {
                    if borders.contains(&(i, j.overflowing_sub(1).0, DIR::RIGHT)) {
                        q.push_back((i, j.overflowing_sub(1).0, DIR::RIGHT));
                    }
                    if borders.contains(&(i, j + 1, DIR::RIGHT)) {
                        q.push_back((i, j + 1, DIR::RIGHT));
                    }
                }
            }
        }
    }
}



