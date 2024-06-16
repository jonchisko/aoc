use crate::aoc_traits::{Solution, SolveAdvent};
use std::fs;

static STRING_TO_NUMBER: [(&'static str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

pub struct Aoc1 {
    pub name: String,
}

impl SolveAdvent for Aoc1 {
    fn compute_solution1(&self) -> Solution {
        let lines = self.read_input();
        let mut result: Vec<i32> = vec![];

        for line in lines {
            let (mut first, mut last) = (None, None);

            for char in line.chars() {
                if char.is_numeric() {
                    if first.is_none() {
                        first = Some(char);
                    }

                    last = Some(char);
                }
            }

            // if any is none, both are none, just continue
            if first.is_none() {
                continue;
            }

            result.push(
                format!("{}{}", first.unwrap(), last.unwrap())
                    .parse::<i32>()
                    .unwrap(),
            );
        }

        Solution::new(&self.name, result.iter().sum::<i32>().to_string())
    }

    fn compute_solution2(&self) -> Solution {
        let lines = self.read_input();
        let mut result: Vec<i32> = vec![];

        for line in lines {
            let mut indexes: Vec<(i32, char)> = vec![];

            for (name, value) in STRING_TO_NUMBER {
                for (match_index, _) in line.match_indices(name) {
                    indexes.push((match_index as i32, value));
                }
            }

            for (index, ch) in line.chars().enumerate() {
                if ch.is_numeric() {
                    indexes.push((index as i32, ch));
                }
            }

            let (mut min_ind, mut min_ch) = (None, None);
            let (mut max_ind, mut max_ch) = (None, None);

            for (index, ch) in indexes {
                if min_ind.is_none() || index < min_ind.unwrap() {
                    min_ind = Some(index);
                    min_ch = Some(ch);
                }

                // rather than else if, just if, so that if just one digit is, it s also the max
                if max_ind.is_none() || index > max_ind.unwrap() {
                    max_ind = Some(index);
                    max_ch = Some(ch);
                }
            }

            // if any is none, both are none, just continue
            if min_ind.is_none() {
                continue;
            }

            // One other way would be to just try to combine the digits and not use string concat.
            result.push(
                format!("{}{}", min_ch.unwrap(), max_ch.unwrap())
                    .parse::<i32>()
                    .unwrap(),
            );
        }

        Solution::new(&self.name, result.iter().sum::<i32>().to_string())
    }
}

impl Aoc1 {
    pub fn new() -> Aoc1 {
        Aoc1 {
            name: "aoc1".to_string(),
        }
    }

    fn read_input(&self) -> Vec<String> {
        fs::read_to_string(format!("inputs/{}.txt", &self.name))
            .expect("No file found.")
            .split('\n')
            .filter(|line| line.len() != 0)
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }
}
