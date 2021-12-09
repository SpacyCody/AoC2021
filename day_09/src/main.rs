use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<String>) {
    let low: Vec<(u16, (usize, usize))> = find_low_and_index(data);
    let mut risk: Vec<u16> = Vec::new();
    for i in 0..low.len() {
        risk.push(low[i].0 + 1);
    }
    println!("{}", risk.iter().sum::<u16>());
}

fn part_b(data: &Vec<String>) {
    let input = create_input(data);
    let low: Vec<(u16, (usize, usize))> = find_low_and_index(data);
    let mut low_indices: Vec<(usize, usize)> = Vec::new();
    for l in low {
        low_indices.push(l.1);
    }
    let mut basins: Vec<Vec<(usize, usize)>> = Vec::new();
    for li in low_indices {
        basins.push(find_basin(&input, li.0, li.1));
    }
    let mut basin_sizes: Vec<usize> = Vec::new();
    for b in basins {
        basin_sizes.push(b.len())
    }
    basin_sizes.sort();
    let mut sum = 1;
    for i in basin_sizes.len() - 3..basin_sizes.len() {
        sum *= basin_sizes[i];
    }
    println!("{}", sum)
}

fn create_input(data: &Vec<String>) -> Vec<Vec<u16>> {
    let mut input: Vec<Vec<u16>> = Vec::new();
    for line in data {
        let mut row: Vec<u16> = Vec::new();
        for c in line.chars() {
            row.push(match c.to_digit(10) {
                Some(n) => n as u16,
                None => 99,
            });
        }
        input.push(row);
    }
    return input;
}

fn find_low_and_index(data: &Vec<String>) -> Vec<(u16, (usize, usize))> {
    let input = create_input(data);
    let mut low: Vec<(u16, (usize, usize))> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if i == 0 {
                if j == 0 {
                    if input[i][j] < input[i + 1][j] && input[i][j] < input[i][j + 1] {
                        low.push((input[i][j], (i, j)));
                    }
                } else if j + 1 == input[i].len() {
                    if input[i][j] < input[i + 1][j] && input[i][j] < input[i][j - 1] {
                        low.push((input[i][j], (i, j)));
                    }
                } else {
                    if input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j - 1]
                        && input[i][j] < input[i][j + 1]
                    {
                        low.push((input[i][j], (i, j)));
                    }
                }
            } else if i + 1 == input.len() {
                if j == 0 {
                    if input[i][j] < input[i - 1][j] && input[i][j] < input[i][j + 1] {
                        low.push((input[i][j], (i, j)));
                    }
                } else if j + 1 == input[i].len() {
                    if input[i][j] < input[i - 1][j] && input[i][j] < input[i][j - 1] {
                        low.push((input[i][j], (i, j)));
                    }
                } else {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i][j - 1]
                        && input[i][j] < input[i][j + 1]
                    {
                        low.push((input[i][j], (i, j)));
                    }
                }
            } else {
                if j == 0 {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j + 1]
                    {
                        low.push((input[i][j], (i, j)));
                    }
                } else if j + 1 == input[i].len() {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j - 1]
                    {
                        low.push((input[i][j], (i, j)));
                    }
                } else {
                    if input[i][j] < input[i - 1][j]
                        && input[i][j] < input[i + 1][j]
                        && input[i][j] < input[i][j - 1]
                        && input[i][j] < input[i][j + 1]
                    {
                        low.push((input[i][j], (i, j)));
                    }
                }
            }
        }
    }
    return low;
}

fn check_surroundings(input: &Vec<Vec<u16>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut new: Vec<(usize, usize)> = Vec::new();
    if i == 0 {
        if j == 0 {
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        } else if j + 1 == input[i].len() {
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
        } else {
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        }
    } else if i + 1 == input.len() {
        if j == 0 {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        } else if j + 1 == input[i].len() {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
        } else {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        }
    } else {
        if j == 0 {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        } else if j + 1 == input[i].len() {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
        } else {
            if input[i - 1][j] != 9 {
                new.push((i - 1, j));
            }
            if input[i + 1][j] != 9 {
                new.push((i + 1, j));
            }
            if input[i][j - 1] != 9 {
                new.push((i, j - 1));
            }
            if input[i][j + 1] != 9 {
                new.push((i, j + 1));
            }
        }
    }
    return new;
}

fn find_basin(input: &Vec<Vec<u16>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut basin: Vec<(usize, usize)> = Vec::new();
    basin.push((i, j));
    let start = check_surroundings(&input, i, j);
    for s in &start {
        basin.push(*s);
    }
    let mut nr_new = start.len();
    while nr_new > 0 {
        let mut new = 0;
        for k in (basin.len() - nr_new)..basin.len() {
            let next = check_surroundings(&input, basin[k].0, basin[k].1);
            if next.len() > 0 {
                for n in next {
                    if !basin.contains(&n) {
                        basin.push(n);
                        new += 1;
                    }
                }
            }
        }
        nr_new = new;
    }
    return basin;
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}
