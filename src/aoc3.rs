use std::{collections::HashSet, fs, num::ParseIntError};

use crate::aoc_traits::{Solution, SolveAdvent};

fn read_input(aoc: &Aoc3) -> String {
    fs::read_to_string(format!("inputs/{}.txt", aoc.name)).expect("Could not read input file.")
}

pub struct Aoc3 {
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
        let input = read_input(self);
        let schematic =
            Schematic::try_from(&input[..]).expect("Could not create Schematic from string");

        Solution::new(&self.name, schematic.part_number_sum().to_string())
    }

    fn compute_solution2(&self) -> Solution {
        todo!();
    }
}

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
struct Point(i32, i32);

#[derive(Debug)]
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
        if !self.symbols.insert(point) {
            panic!("Point was already contained");
        }
    }

    pub fn add_number(&mut self, number: Number) {
        self.numbers.push(number);
    }

    pub fn part_number_sum(&self) -> i32 {
        let mut sum: i32 = 0;

        for number in &self.numbers {
            if !self.is_part(&number) {
                continue;
            }
            sum += number.0;
        }

        sum
    }

    fn is_part(&self, part_number: &Number) -> bool {
        for location in &part_number.1 {
            // nice nesting depth, if i'd concern myself with clean code at this place, I'd rework this.
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let point_query = Point(location.0 + i, location.1 + j);
                    if self.symbols.contains(&point_query) {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn schematic_construction() {
        // Arrange
        let input = "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..";

        // Act
        let schematic = Schematic::try_from(input).unwrap();

        // Assert
        assert_eq!(
            schematic.numbers.len(),
            10,
            "There should be 10 numbers in the schematic."
        );
        assert_eq!(
            schematic.symbols.len(),
            6,
            "There should be 6 symbols in the schematic."
        );
        assert_eq!(
            schematic.numbers[0].1,
            vec![Point(0, 0), Point(1, 0), Point(2, 0)],
            "Coordinates are not correct"
        );
        assert_eq!(
            schematic.part_number_sum(),
            4361,
            "Part numbers should sum to 4361."
        );
    }
}
