use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<u16>) {
    let mut old: u16 = 0;
    let mut first = true;
    let mut nr = 0;
    for number in data {
        if first {
            old = *number;
            first = false;
        } else {
            let new = *number;
            if old < new {
                nr += 1;
            }
            old = new;
        }
    }
    println!("{}", nr);
}

fn part_b(data: &Vec<u16>) {
    let mut three_sum: Vec<u16> = Vec::new();
    for i in 0..(data.len() - 2) {
        three_sum.push(data[i] + data[i + 1] + data[i + 2]);
    }
    part_a(&three_sum);
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<u16> = Vec::new();

    for line in reader.lines() {
        data.push(line?.parse::<u16>().unwrap());
    }
    println!("Part A:");
    part_a(&data);
    println!("Part A:");
    part_b(&data);
    Ok(())
}
