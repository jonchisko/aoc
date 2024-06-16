mod aoc1;
mod aoc_traits;

use crate::aoc1::Aoc1;
use crate::aoc_traits::SolveAdvent;

fn main() {
    let aoc1 = Aoc1::new();
    aoc1.solve1();
    aoc1.solve2();
}
