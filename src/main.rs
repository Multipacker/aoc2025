use std::fs;
use std::error::Error;

pub mod day1;
pub mod day2;

use crate::day1::*;
use crate::day2::*;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day2.txt")?;
    day2(&data);
    return Ok(())
}
