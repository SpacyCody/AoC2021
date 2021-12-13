use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<Vec<u8>>) {
    let (nr_of_glows, _) = run(&data, 100, false);
    println!("{}", nr_of_glows);
}

fn part_b(data: &Vec<Vec<u8>>) {
    let (_, runs) = run(&data, 1000, true);
    println!("All glows att run: {}", runs)
}

fn run(data: &Vec<Vec<u8>>, runs: u32, looking_for_all_glow: bool) -> (usize, i32) {
    let mut input = data.to_vec();
    let mut glow_octos: Vec<(usize, usize)> = Vec::new();
    let mut nr_of_glows = 0;
    let mut all_glow_run = -1;
    for r in 0..runs {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                input[i][j] += 1;
            }
        }
        let mut not_done = true;
        while not_done {
            for i in 0..input.len() {
                for j in 0..input[i].len() {
                    if input[i][j] > 9 && !glow_octos.contains(&(i, j)) {
                        glow_octos.push((i, j));
                        if i == 0 {
                            if j == 0 {
                                input[i + 1][j] += 1;
                                input[i][j + 1] += 1;
                                input[i + 1][j + 1] += 1;
                            } else if j + 1 == input[i].len() {
                                input[i + 1][j] += 1;
                                input[i][j - 1] += 1;
                                input[i + 1][j - 1] += 1;
                            } else {
                                input[i][j - 1] += 1;
                                input[i + 1][j - 1] += 1;
                                input[i + 1][j] += 1;
                                input[i + 1][j + 1] += 1;
                                input[i][j + 1] += 1;
                            }
                        } else if i + 1 == input.len() {
                            if j == 0 {
                                input[i - 1][j] += 1;
                                input[i][j + 1] += 1;
                                input[i - 1][j + 1] += 1;
                            } else if j + 1 == input[i].len() {
                                input[i - 1][j] += 1;
                                input[i][j - 1] += 1;
                                input[i - 1][j - 1] += 1;
                            } else {
                                input[i][j - 1] += 1;
                                input[i - 1][j - 1] += 1;
                                input[i - 1][j] += 1;
                                input[i - 1][j + 1] += 1;
                                input[i][j + 1] += 1;
                            }
                        } else {
                            if j == 0 {
                                input[i - 1][j] += 1;
                                input[i - 1][j + 1] += 1;
                                input[i][j + 1] += 1;
                                input[i + 1][j + 1] += 1;
                                input[i + 1][j] += 1;
                            } else if j + 1 == input[i].len() {
                                input[i][j - 1] += 1;
                                input[i - 1][j - 1] += 1;
                                input[i - 1][j] += 1;
                                input[i + 1][j - 1] += 1;
                                input[i + 1][j] += 1;
                            } else {
                                input[i - 1][j - 1] += 1;
                                input[i - 1][j] += 1;
                                input[i - 1][j + 1] += 1;
                                input[i][j + 1] += 1;
                                input[i + 1][j + 1] += 1;
                                input[i + 1][j] += 1;
                                input[i + 1][j - 1] += 1;
                                input[i][j - 1] += 1;
                            }
                        }
                    }
                }
            }
            not_done = any_ready_to_glow(&input, &glow_octos);
        }
        nr_of_glows += glow_octos.len();
        if looking_for_all_glow && glow_octos.len() == 100 {
            all_glow_run = (r + 1) as i32;
            break;
        }
        for oct in &glow_octos {
            input[oct.0][oct.1] = 0;
        }
        glow_octos.clear();
    }
    return (nr_of_glows, all_glow_run);
}

fn any_ready_to_glow(input: &Vec<Vec<u8>>, glowing: &Vec<(usize, usize)>) -> bool {
    let mut result = false;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] > 9 && !glowing.contains(&(i, j)) {
                result = true;
            }
        }
    }
    return result;
}

fn _print_octos(data: &Vec<Vec<u8>>) {
    for i in 0..data.len() {
        for j in 0..data.len() {
            print!("{:3}", data[i][j])
        }
        print!("\n")
    }
    println!("");
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<Vec<u8>> = Vec::new();
    for r in reader.lines() {
        let line = r?;
        let mut line_nr: Vec<u8> = Vec::new();
        for c in line.chars() {
            line_nr.push(match c.to_digit(10) {
                Some(n) => n as u8,
                None => 99,
            });
        }
        data.push(line_nr);
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}
