use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(starts: &Vec<(usize, usize)>, ends: &Vec<(usize, usize)>) {
    let mut map = create_matrix(1000);
    let mut crossing = 0;
    println!(
        "starts length: {}, ends length: {}",
        starts.len(),
        ends.len()
    );
    for i in 0..starts.len() {
        let x1 = starts[i].0;
        let x2 = ends[i].0;
        let y1 = starts[i].1;
        let y2 = ends[i].1;
        let mut list: Vec<(usize, usize)> = Vec::new();
        if y1 == y2 {
            if x1 > x2 {
                for x in x2..x1 + 1 {
                    list.push((x, y1));
                    map[y1][x] += 1;
                }
            } else {
                for x in x1..x2 + 1 {
                    list.push((x, y1));
                    map[y1][x] += 1;
                }
            }
        } else if x1 == x2 {
            if y1 > y2 {
                for y in y2..y1 + 1 {
                    if !list.contains(&(x1, y)) {
                        map[y][x1] += 1;
                    }
                }
            } else {
                for y in y1..y2 + 1 {
                    if !list.contains(&(x1, y)) {
                        map[y][x1] += 1;
                    }
                }
            }
        }
    }
    for i in 0..1000 {
        for j in 0..1000 {
            if map[i][j] > 1 {
                crossing += 1;
            }
        }
    }
    println!("{}", crossing)
}

fn part_b(starts: &Vec<(usize, usize)>, ends: &Vec<(usize, usize)>) {
    let mut map = create_matrix(1000);
    let mut crossing = 0;
    for i in 0..starts.len() {
        let x1 = starts[i].0;
        let x2 = ends[i].0;
        let y1 = starts[i].1;
        let y2 = ends[i].1;
        let mut list: Vec<(usize, usize)> = Vec::new();
        if y1 == y2 {
            if x1 > x2 {
                for x in x2..x1 + 1 {
                    list.push((x, y1));
                    map[y1][x] += 1;
                }
            } else {
                for x in x1..x2 + 1 {
                    list.push((x, y1));
                    map[y1][x] += 1;
                }
            }
        } else if x1 == x2 {
            if y1 > y2 {
                for y in y2..y1 + 1 {
                    if !list.contains(&(x1, y)) {
                        map[y][x1] += 1;
                    }
                }
            } else {
                for y in y1..y2 + 1 {
                    if !list.contains(&(x1, y)) {
                        map[y][x1] += 1;
                    }
                }
            }
        } else if diff(x1, x2) == diff(y1, y2) {
            let mut start_x = x1;
            let end_x = x2;
            let mut start_y = y1;
            if x1 > x2 && y1 > y2 {
                for _i in 0..diff(start_x, end_x) + 1 {
                    map[start_y][start_x] += 1;
                    if start_x != 0 && start_y != 0 {
                        start_x -= 1;
                        start_y -= 1;
                    }
                }
            } else if x1 > x2 {
                for _i in 0..diff(start_x, end_x) + 1 {
                    map[start_y][start_x] += 1;
                    if start_x != 0 {
                        start_x -= 1;
                    }
                    start_y += 1;
                }
            } else if y1 > y2 {
                for _i in 0..diff(start_x, end_x) + 1 {
                    map[start_y][start_x] += 1;
                    start_x += 1;
                    if start_y != 0 {
                        start_y -= 1;
                    }
                }
            } else {
                for _i in 0..diff(start_x, end_x) + 1 {
                    map[start_y][start_x] += 1;
                    start_x += 1;
                    start_y += 1;
                }
            }
        }
    }
    for i in 0..1000 {
        for j in 0..1000 {
            if map[i][j] > 1 {
                crossing += 1;
            }
        }
    }
    println!("{}", crossing)
}

fn create_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = Vec::new();
    for _i in 0..size {
        let row: Vec<usize> = vec![0; 1000];
        matrix.push(row);
    }
    return matrix;
}

fn diff(a: usize, b: usize) -> usize {
    if a < b {
        return b - a;
    } else {
        return a - b;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut start_cords: Vec<(usize, usize)> = Vec::new();
    let mut end_cords: Vec<(usize, usize)> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let cords: Vec<&str> = l.split(" -> ").collect();
        start_cords.push(string_to_cords(cords[0]));
        end_cords.push(string_to_cords(cords[1]));
    }

    println!("Part A");
    part_a(&start_cords, &end_cords);
    println!("Part B");
    part_b(&start_cords, &end_cords);

    Ok(())
}

fn string_to_cords(s: &str) -> (usize, usize) {
    let values: Vec<&str> = s.split(",").collect();
    let cords: (usize, usize) = (
        values[0].parse::<usize>().unwrap(),
        values[1].parse::<usize>().unwrap(),
    );
    return cords;
}

fn _print_matrix(data: Vec<Vec<usize>>) {
    for i in 0..10 {
        for j in 0..10 {
            print!("{:3}", data[i][j])
        }
        print!("\n")
    }
}
