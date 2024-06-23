use std::{collections::HashSet, fs, num::ParseIntError};

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

enum SchematicError {
    MalformedString,
    IntConversion(ParseIntError),
}

impl TryFrom<&str> for Schematic {
    type Error = SchematicError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut schematic = Schematic::new();

        let mut number: Option<i32> = None;
        let mut coordinates: Vec<Point> = vec![];

        let lines = value.split('\n');
        for (y, line) in lines.enumerate() {
            let line = line.trim();

            for (x, ch) in line.chars().enumerate() {
                // rework this to match if possible?!
                if ch.is_digit(10u32) {
                    number = match number {
                        Some(n) => Some(10 * n + ch.to_digit(10).unwrap() as i32),
                        None => Some(ch.to_digit(10).unwrap() as i32),
                    };

                    coordinates.push(Point(x as i32, y as i32));
                } else {
                    if number.is_some() {
                        schematic.add_number(Number(number.take().unwrap(), coordinates.clone()));
                        coordinates.clear();
                    }
                    if !ch.is_alphabetic() && ch != '.' {
                        schematic.add_symbol(Point(x as i32, y as i32));
                    }
                }
            }

            // could also if let Some(n)
            if number.is_some() {
                schematic.add_number(Number(number.take().unwrap(), coordinates.clone()));
                coordinates.clear();
            }
        }

        Ok(schematic)
    }
}
