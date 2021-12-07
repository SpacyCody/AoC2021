use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<u32>) {
    let mut fishes: Vec<u32> = data.to_vec();
    for _d in 0..80 {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
    }
    println!("Fishes: {}", fishes.len())
}

fn part_b(data: &str) {
    println!("Fishes: {}", solve(data, 256));
}

fn solve(data: &str, days: u32) -> u64 {
    let mut days_to_birth = [0; 9];

    data.split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .for_each(|age| days_to_birth[age] += 1);

    for _ in 1..=days {
        days_to_birth.rotate_left(1);
        days_to_birth[6] += days_to_birth[8];
    }
    return days_to_birth.iter().sum();
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut fishes: Vec<u32> = Vec::new();
    let mut data: String = String::new();
    for line in reader.lines() {
        let l = line?;
        data = l;
        let input: Vec<&str> = data.split(",").collect();
        for f in input {
            fishes.push(f.parse::<u32>().unwrap())
        }
    }

    println!("Part A");
    part_a(&fishes);
    println!("Part B");
    part_b(&data);

    Ok(())
}
