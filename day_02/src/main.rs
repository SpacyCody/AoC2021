use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<(String, u32)>) {
    let mut dist = 0;
    let mut depth = 0;
    for command in data {
        let (x, z) = command;
        if x == "forward" {
            dist += z;
        } else if x == "down" {
            depth += z;
        } else if x == "up" {
            depth -= z;
        } else {
            println!("ERROR!!!");
        }
    }
    println!("{}", dist * depth);
}

fn part_b(data: &Vec<(String, u32)>) {
    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in data {
        let (x, z) = command;
        if x == "forward" {
            dist += z;
            depth += aim * z
        } else if x == "down" {
            aim += z;
        } else if x == "up" {
            aim -= z;
        } else {
            println!("ERROR!!!");
        }
    }
    println!("{}", dist * depth);
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<(String, u32)> = Vec::new();
    for line in reader.lines() {
        let input: String = line?;
        let split: Vec<&str> = input.split(" ").collect();
        let dir = String::from(split[0]);
        let dist = split[1].parse::<u32>().unwrap();
        data.push((dir, dist));
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}
