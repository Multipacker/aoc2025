use std::fs;
use std::error::Error;

pub mod day1;

use crate::day1::*;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day1.txt")?;
    day1(&data);
    return Ok(())
}
