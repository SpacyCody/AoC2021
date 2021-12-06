use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(res: &String, boards_str: &Vec<String>) {
    let data = string_to_vec_of_num(res);
    let mut boards: Vec<[[(u32, bool); 5]; 5]> = Vec::new();
    let mut bingos: Vec<bool> = Vec::new();
    let mut last_drawn = 0;
    let mut bingo_index = 99;
    for board in boards_str {
        boards.push(string_to_board(&board));
    }

    for _i in 0..boards.len() {
        bingos.push(false);
    }

    for d in data {
        let mut i = 0;
        last_drawn = d;
        for board in boards.iter_mut() {
            let index = find_index_for_number(*board, d);
            if index.0 < 5 {
                board[index.0][index.1].1 = true;
            }
            bingos[i] = check_if_bingo(*board);
            i += 1;
        }
        bingo_index = find_bingo_board(&bingos);
        if bingo_index != 99 {
            break;
        }
    }
    let unmarked_sum = find_sum_of_unmarked(boards[bingo_index]);
    print_board(boards[bingo_index]);
    println!("{}", unmarked_sum * last_drawn);
}

fn part_b(res: &String, boards_str: &Vec<String>) {
    let data = string_to_vec_of_num(res);
    let mut boards: Vec<[[(u32, bool); 5]; 5]> = Vec::new();
    let mut bingos: Vec<bool> = Vec::new();
    let mut last_drawn = 0;
    let mut last_board_index: usize = 0;
    for board in boards_str {
        boards.push(string_to_board(&board));
    }

    for _i in 0..boards.len() {
        bingos.push(false);
    }

    for d in data {
        let mut i = 0;
        last_drawn = d;
        for board in boards.iter_mut() {
            let index = find_index_for_number(*board, d);
            if index.0 < 5 {
                board[index.0][index.1].1 = true;
            }
            bingos[i] = check_if_bingo(*board);
            i += 1;
        }
        let no_bingo_indices = list_not_bingo_yet(&bingos);
        if no_bingo_indices.len() == 1 {
            last_board_index = no_bingo_indices[0];
        } else if no_bingo_indices.len() == 0 {
            break;
        }
    }
    let unmarked_sum = find_sum_of_unmarked(boards[last_board_index]);
    print_board(boards[last_board_index]);
    println!("{}", unmarked_sum * last_drawn);
}

fn string_to_vec_of_num(strings: &String) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    for s in strings.split(",") {
        results.push(s.parse::<u32>().unwrap());
    }
    return results;
}

fn string_to_board(data: &String) -> [[(u32, bool); 5]; 5] {
    let mut board: [[(u32, bool); 5]; 5] = [
        [(0, false), (0, false), (0, false), (0, false), (0, false)],
        [(0, false), (0, false), (0, false), (0, false), (0, false)],
        [(0, false), (0, false), (0, false), (0, false), (0, false)],
        [(0, false), (0, false), (0, false), (0, false), (0, false)],
        [(0, false), (0, false), (0, false), (0, false), (0, false)],
    ];
    let mut i = 0;
    for s in data.split("\n") {
        if i < 5 {
            let mut j = 0;
            for num in s.split_whitespace() {
                board[i][j].0 = num.parse::<u32>().unwrap();
                j += 1;
            }
            i += 1;
        }
    }
    return board;
}

fn find_index_for_number(board: [[(u32, bool); 5]; 5], nr: u32) -> (usize, usize) {
    let mut index: (usize, usize) = (11, 11);
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].0 == nr {
                index = (i, j)
            }
        }
    }
    return index;
}

fn check_if_bingo(board: [[(u32, bool); 5]; 5]) -> bool {
    let mut bingo = false;
    for i in 0..5 {
        if board[i][0].1 && board[i][1].1 && board[i][2].1 && board[i][3].1 && board[i][4].1 {
            bingo = true;
        } else if board[0][i].1 && board[1][i].1 && board[2][i].1 && board[3][i].1 && board[4][i].1
        {
            bingo = true;
        }
    }
    return bingo;
}

fn find_bingo_board(bingos: &Vec<bool>) -> usize {
    let mut index = 99;
    for i in 0..bingos.len() {
        if bingos[i] {
            index = i;
            break;
        }
    }
    return index;
}

fn find_sum_of_unmarked(board: [[(u32, bool); 5]; 5]) -> u32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board[i][j].1 {
                sum += board[i][j].0
            }
        }
    }
    return sum;
}

fn print_board(board: [[(u32, bool); 5]; 5]) {
    for i in 0..5 {
        for j in 0..5 {
            let mut s = String::new();
            if board[i][j].1 {
                s.push_str(&board[i][j].0.to_string());
                s.push_str("*");
            } else {
                s.push_str(&board[i][j].0.to_string());
            }
            print!("{:5}", s);
        }
        print!("\n")
    }
}

fn list_not_bingo_yet(bingos: &Vec<bool>) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();
    for i in 0..bingos.len() {
        if !bingos[i] {
            list.push(i);
        }
    }
    return list;
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut result_str: String = String::new();
    let mut boards_str: Vec<String> = Vec::new();
    let mut i = 0;
    let mut tmp_str: String = String::new();
    for line in reader.lines() {
        if i == 0 {
            result_str = line?;
        } else if (i % 6) == 1 {
            let tmp = &tmp_str;
            if tmp.len() != 0 {
                boards_str.push(tmp.to_string())
            }
            tmp_str.clear();
        } else {
            tmp_str.push_str(&line?);
            tmp_str.push_str("\n");
        }
        i += 1;
    }

    println!("Part A");
    part_a(&result_str, &boards_str);
    println!("Part B");
    part_b(&result_str, &boards_str);

    Ok(())
}
