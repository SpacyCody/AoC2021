fn part_a(data: &str) {
    let mut paper = create_paper(data);
    let instructions = get_instructions(data);
    let (p, i) = instructions[0];
    if p.eq(&'x') {
        paper = fold(i, paper.len(), paper);
    }
    
    println!("{}", count_dots(paper))
}

fn part_b(data: &str) {
    let mut paper = create_paper(data);
    let instructions = get_instructions(data);
    for ins in instructions {
        let (p, i) = ins;
        if p.eq(&'x') {
            paper = fold(i, paper.len(), paper);
        } else {
            paper = fold(paper[0].len(), i, paper);
        }
    }
    println!("y size: {}, x size: {}", paper.len(), paper[0].len());
    // println!("{}", count_dots(paper))
    print_paper(paper);
}

fn main() {
    let data = include_str!("../data.txt");
    println!("Part A");
    part_a(data);
    println!("Part B");
    part_b(data);
}

fn create_paper(data: &str) -> Vec<Vec<char>> {
    let mut paper: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for _ in 0..1311 {
        row.push('.');
    }
    let r = row;
    for _ in 0..895 {
        paper.push(r.to_vec())
    }
    for line in data.lines() {
        if line.contains(",") {
            let (x, y) = line.split_once(",").unwrap();
            paper[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';
        }
    }
    return paper;
}

fn get_instructions(data: &str) -> Vec<(char, usize)> {
    let mut instructions: Vec<(char, usize)> = Vec::new();
    for line in data.lines() {
        if line.contains("=") {
            let tmp: Vec<&str> = line.split_whitespace().collect();
            let (c, i) = tmp[2].split_once("=").unwrap();
            instructions.push((c.parse::<char>().unwrap(), i.parse::<usize>().unwrap()))
        }
    }
    return instructions;
}

fn fold(x_size: usize, y_size: usize, paper: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut folded_paper: Vec<Vec<char>> = Vec::new();
    if paper.len() == y_size {
        for y in 0..y_size {
            let mut row: Vec<char> = Vec::new();
            for x in 0..x_size {
                if paper[y][x].eq(&'#') || paper[y][(paper[y].len() - 1) - x].eq(&'#') {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            folded_paper.push(row);
        }
    } else {
        for y in 0..y_size {
            let mut row: Vec<char> = Vec::new();
            for x in 0..x_size {
                if paper[y][x].eq(&'#') || paper[(paper.len() - 1) - y][x].eq(&'#') {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            folded_paper.push(row);
        }
    }
    return folded_paper
}

fn count_dots(paper: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for y in 0..paper.len() {
        for x in 0..paper[y].len() {
            if paper[y][x].eq(&'#') {
                count += 1;
            }
        }
    }
    return count;
}

fn print_paper(paper: Vec<Vec<char>>) {
    for y in 0..paper.len() {
        for x in 0..paper[y].len() {
            print!("{:3}", paper[y][x])
        }
        print!("\n");
    }
}