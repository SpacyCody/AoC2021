use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &str) {
    let start_positions: Vec<u32> = data
        .split(",")
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    let mut fuel_costs: Vec<u32> = Vec::new();
    let max_value: u32 = match start_positions.iter().max() {
        Some(max) => *max,
        None => 0,
    };
    for i in 0..max_value + 1 {
        let mut fuel_cost = 0;
        for pos in &start_positions {
            fuel_cost += count_fuel_a(*pos, i);
        }
        fuel_costs.push(fuel_cost);
    }
    match fuel_costs.iter().min() {
        Some(min) => println!("{}", min),
        None => println!("ERROR"),
    };
}

fn count_fuel_a(pos: u32, aim: u32) -> u32 {
    return if pos > aim { pos - aim } else { aim - pos };
}

fn part_b(data: &str) {
    let start_positions: Vec<u32> = data
        .split(",")
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    let mut fuel_costs: Vec<u32> = Vec::new();
    let max_value: u32 = match start_positions.iter().max() {
        Some(max) => *max,
        None => 0,
    };
    for i in 0..max_value + 1 {
        let mut fuel_cost = 0;
        for pos in &start_positions {
            fuel_cost += count_fuel_b(*pos, i);
        }
        fuel_costs.push(fuel_cost);
    }
    match fuel_costs.iter().min() {
        Some(min) => println!("{}", min),
        None => println!("ERROR"),
    };
}

fn count_fuel_b(pos: u32, aim: u32) -> u32 {
    let start: u32;
    let end: u32;
    if pos > aim {
        start = aim;
        end = pos;
    } else {
        start = pos;
        end = aim;
    };
    let mut result = 0;
    for i in 1..end - start + 1 {
        result += i;
    }
    return result;
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: String = String::new();
    for line in reader.lines() {
        let l = line?;
        data = l;
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}
