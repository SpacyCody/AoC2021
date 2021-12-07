use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<Vec<u32>>) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    let nr_of_ones = find_number_of_ones(&data);
    for pos in 0..nr_of_ones.len() {
        if nr_of_ones[pos] < (data.len() / 2).try_into().unwrap() {
            epsilon += base.pow((nr_of_ones.len() - 1 - pos).try_into().unwrap());
        } else {
            gamma += base.pow((nr_of_ones.len() - 1 - pos).try_into().unwrap());
        }
    }
    println!("{}", gamma * epsilon)
}

fn part_b(data: &Vec<Vec<u32>>) {
    let oxygen = oxygen_rating(data);
    let co2 = co2_rating(data);
    println!("{}", oxygen * co2)
}

fn find_number_of_ones(data: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut ones: Vec<u32> = vec![0; 12];
    for d in data {
        for i in 0..d.len() {
            if d[i] == 1 {
                ones[i] += 1
            }
        }
    }
    return ones;
}

fn oxygen_rating(data: &Vec<Vec<u32>>) -> u32 {
    let oxygen: u32;
    let mut o_list: Vec<Vec<u32>> = data.to_vec();
    while o_list.len() != 1 {
        let tmp: Vec<Vec<u32>> = process_oxygen_vec(&o_list);
        o_list.clear();
        o_list = tmp;
    }
    oxygen = bin_vec_to_dec(&o_list[0]);
    return oxygen;
}

fn process_oxygen_vec(data: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let length = data[0].len();
    let mut new_data: Vec<Vec<u32>> = data.to_vec();
    for i in 0..length {
        let tmp = &new_data.to_vec();
        if new_data.len() == 1 {
            break;
        }
        let ones = find_number_of_ones(&new_data);
        new_data.clear();
        let pick: u32;
        let size: u32 = tmp.len().try_into().unwrap();
        if ones[i] < (size - ones[i]) {
            pick = 0;
        } else {
            pick = 1;
        }
        for input in tmp {
            if input[i] == pick {
                new_data.push(input.to_vec())
            }
        }
    }
    return new_data;
}

fn co2_rating(data: &Vec<Vec<u32>>) -> u32 {
    let co2: u32;
    let mut co2_list: Vec<Vec<u32>> = data.to_vec();
    let mut length;
    loop {
        let tmp: Vec<Vec<u32>> = process_co2_vec(&co2_list);
        length = tmp.len();
        co2_list.clear();
        co2_list = tmp;
        if length == 1 {
            break;
        }
    }
    co2 = bin_vec_to_dec(&co2_list[0]);
    return co2;
}

fn process_co2_vec(data: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let length = data[0].len();
    let mut new_data: Vec<Vec<u32>> = data.to_vec();
    for i in 0..length {
        let tmp = &new_data.to_vec();
        if new_data.len() == 1 {
            break;
        }
        let ones = find_number_of_ones(&new_data);
        new_data.clear();
        let pick: u32;
        let size: u32 = tmp.len().try_into().unwrap();
        if ones[i] < (size - ones[i]) {
            pick = 1;
        } else {
            pick = 0;
        }
        for input in tmp {
            if input[i] == pick {
                new_data.push(input.to_vec())
            }
        }
    }
    return new_data;
}

fn bin_vec_to_dec(data: &Vec<u32>) -> u32 {
    let mut dec: u32 = 0;
    let base: u32 = 2;
    for pos in 0..data.len() {
        dec += data[pos] * base.pow((data.len() - 1 - pos).try_into().unwrap());
    }
    return dec;
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let mut input: Vec<u32> = Vec::new();
        for c in line?.chars() {
            input.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
        data.push(input);
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}

fn _print_vec(data: &Vec<u32>) {
    for d in data {
        print!("{} ", d)
    }
    print!("\n")
}
