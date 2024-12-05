pub trait Solver<T: std::fmt::Display , U: std::fmt::Display> {
    fn solve_part1(&self) -> T;
    fn solve_part2(&self) -> U;
    fn solve(&self) -> () {
        println!("part1: {}, part2: {}", self.solve_part1(), self.solve_part2());
    }
}