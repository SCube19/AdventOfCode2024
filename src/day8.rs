
use std::fs;
use crate::solver::Solver;
use std::collections::{HashMap, HashSet};
pub struct Day8;
impl Solver<usize, usize> for Day8 {
    fn solve_part1(&self) -> usize {
        let file: String = fs::read_to_string("input/day8.txt").unwrap();
        let grid: Vec<Vec<char>> = file.lines().map(|c| c.chars().collect()).collect::<Vec<Vec<_>>>();

        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '.' {
                    antennas.entry(grid[i][j]).or_insert_with(Vec::new).push((i as i32, j as i32));
                }
            }
        }

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();        
        for antenna_type in antennas.values() {
            for &first in antenna_type {
                for &second in antenna_type {
                    if first == second {
                        continue;
                    }
                    let diff: (i32, i32) = (first.0 - second.0, first.1 - second.1);
                    let antinode: (i32, i32)  = (first.0 + diff.0, first.1 + diff.1);

                    if antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < grid.len() as i32 && antinode.1 < grid[0].len() as i32 {
                        antinodes.insert(antinode);
                    }                    
                }
            }
        }
        return antinodes.len();
    }

    fn solve_part2(&self) -> usize {
        let file: String = fs::read_to_string("input/day8.txt").unwrap();
        let grid: Vec<Vec<char>> = file.lines().map(|c| c.chars().collect()).collect::<Vec<Vec<_>>>();

        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '.' {
                    antennas.entry(grid[i][j]).or_insert_with(Vec::new).push((i as i32, j as i32));
                }
            }
        }

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();        
        for antenna_type in antennas.values() {
            for &first in antenna_type {
                for &second in antenna_type {
                    if first == second {
                        continue;
                    }
                    let diff: (i32, i32) = (first.0 - second.0, first.1 - second.1);
                    //lazy solution (as in I'm being lazy)
                    let mut antinode: (i32, i32)  = first.clone();
                    
                    while antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < grid.len() as i32 && antinode.1 < grid[0].len() as i32 {
                        antinodes.insert(antinode);
                        antinode = (antinode.0 + diff.0, antinode.1 + diff.1);
                    }                    
                }
            }
        }
        return antinodes.len();
    }
}
