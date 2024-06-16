use std::fs;

pub trait SolveAdvent {
    fn solve1(&self) {
        let solution = SolveAdvent::compute_solution1(self);
        <Self as SolveAdvent>::write_to_file(1, &solution);
    }

    fn solve2(&self) {
        let solution = SolveAdvent::compute_solution2(self);
        <Self as SolveAdvent>::write_to_file(2, &solution);
    }

    fn write_to_file(id: u8, solution: &Solution) {
        fs::write(
            format!("{}_{}.txt", &solution.file_name, id),
            &solution.output_data,
        )
        .expect(&format!("Could not write file {}", &solution.file_name));
    }

    fn compute_solution1(&self) -> Solution;
    fn compute_solution2(&self) -> Solution;
}

pub struct Solution<'a> {
    file_name: &'a str,
    output_data: String,
}

impl<'a> Solution<'a> {
    pub fn new(file_name: &'a str, output_data: String) -> Solution<'a> {
        Solution {
            file_name,
            output_data,
        }
    }
}
