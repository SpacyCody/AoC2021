use std::fs::File;
use std::io::{self, prelude::*, BufReader};

static OPEN: [char; 4] = ['(', '[', '{', '<'];
static CLOSE: [char; 4] = [')', ']', '}', '>'];
static POINTS: [u32; 4] = [3, 57, 1197, 25137];
static SCORE: [u32; 4] = [1, 2, 3, 4];

fn part_a(data: &Vec<String>) {
    let mut indices: Vec<usize> = Vec::new();
    let mut points: Vec<u32> = Vec::new();
    for line in data {
        let (i, p) = find_corrupted(line);
        indices.push(i);
        points.push(p);
    }
    println!("{}", points.iter().sum::<u32>());
}

fn part_b(data: &Vec<String>) {
    let mut missing_list: Vec<String> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();
    for line in data {
        let (tmp, _) = find_corrupted(line);
        if tmp == 999 {
            missing_list.push(find_missing_pairs(line));
        }
    }
    for missing in missing_list {
        scores.push(calculate_autocomplete_score(&missing))
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2])
}

fn find_corrupted(input: &String) -> (usize, u32) {
    let mut index = 999;
    let mut points = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut chunk: Vec<char> = Vec::new();
    for i in 0..chars.len() {
        if OPEN.contains(&chars[i]) {
            chunk.push(chars[i]);
        } else if CLOSE.contains(&chars[i]) {
            if matching_pair(&chunk[chunk.len() - 1], &chars[i]) {
                chunk.pop();
            } else {
                println!();
                index = i;
                points = find_points(chars[i]);
                break;
            }
        }
    }
    return (index, points);
}

fn find_missing_pairs(input: &String) -> String {
    let mut missing: String = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut chunk: Vec<char> = Vec::new();
    for i in 0..chars.len() {
        if OPEN.contains(&chars[i]) {
            chunk.push(chars[i]);
        } else if CLOSE.contains(&chars[i]) {
            if matching_pair(&chunk[chunk.len() - 1], &chars[i]) {
                chunk.pop();
            }
        }
    }
    if chunk.len() > 0 {
        let mut index = chunk.len() - 1;
        for _ in 0..chunk.len() {
            missing.push(CLOSE[find_open_index(chunk[index])]);
            if index > 0 {
                index -= 1;
            }
        }
    }
    return missing;
}

fn matching_pair(a: &char, b: &char) -> bool {
    let mut result = true;
    for i in 0..OPEN.len() {
        if a.eq(&OPEN[i]) {
            result = b.eq(&CLOSE[i]);
        }
    }
    return result;
}

fn find_points(c: char) -> u32 {
    let mut points = 0;
    for i in 0..POINTS.len() {
        if c.eq(&CLOSE[i]) {
            points = POINTS[i];
        }
    }
    return points;
}

fn calculate_autocomplete_score(input: &String) -> u64 {
    let mut score: u64 = 0;
    let chars: Vec<char> = input.chars().collect();
    for c in chars {
        let tmp = score;
        score = tmp * 5 + SCORE[find_close_index(c)] as u64;
    }
    return score;
}

fn find_open_index(co: char) -> usize {
    let mut index = 99;
    for i in 0..OPEN.len() {
        if co.eq(&OPEN[i]) {
            index = i;
            break;
        }
    }
    return index;
}

fn find_close_index(co: char) -> usize {
    let mut index = 99;
    for i in 0..CLOSE.len() {
        if co.eq(&CLOSE[i]) {
            index = i;
            break;
        }
    }
    return index;
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
