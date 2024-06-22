use std::{collections::HashSet, fs};

use crate::aoc_traits::{Solution, SolveAdvent};

struct Aoc3 {
    name: String,
}

impl Aoc3 {
    pub fn new() -> Aoc3 {
        Aoc3 {
            name: "aoc3".to_string(),
        }
    }
}

impl SolveAdvent for Aoc3 {
    fn compute_solution1(&self) -> Solution {
        todo!();
    }

    fn compute_solution2(&self) -> Solution {
        todo!();
    }
}

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct Point(i32, i32);

struct Number(i32, Vec<Point>); // memory footprint might be made smaller, by using some smart pointer to a slice.
                                // ... since it will mostly be read and not written to, after the construction

struct Schematic {
    symbols: HashSet<Point>,
    numbers: Vec<Number>,
}

impl Schematic {
    pub fn new() -> Schematic {
        Schematic {
            symbols: HashSet::new(),
            numbers: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, point: Point) {
        if self.symbols.insert(point) {
            panic!("Point was already contained");
        }
    }

    pub fn add_number(&mut self, number: Number) {
        self.numbers.push(number);
    }
}
