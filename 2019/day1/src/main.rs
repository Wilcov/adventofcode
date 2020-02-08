use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let answer: f32 = reader.lines().map(|v| { required_fuel(v.unwrap())}).sum();
    println!("{}", answer)
}

// Fuel required to launch a given module is based on its mass. 
// Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
fn required_fuel(m: String) -> f32 {
    let mass: f32 = m.parse().unwrap();
    (mass / 3.0).floor() - 2.0
}