use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part_a(data: &str) {
    let map: Vec<Vec<_>> = data
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let goal = (map[0].len() as i32 - 1, map.len() as i32 - 1);

    let risk = find_lowest_risk(&map, &goal);
    println!("Lowest risk: {}", risk);
}

fn part_b(data: &str) {
    let map: Vec<Vec<_>> = expand_map(
        &data
            .lines()
            .map(|l| l.bytes().map(|c| c - b'0').collect())
            .collect(),
    );
    let goal = (map[0].len() as i32 - 1, map.len() as i32 - 1);

    let risk = find_lowest_risk(&map, &goal);

    println!("Lowest risk: {}", risk);
}

fn main() {
    let data = include_str!("../data.txt");
    println!("Part A");
    part_a(data);
    println!("Part B");
    part_b(data);
}

fn find_lowest_risk(map: &Vec<Vec<u8>>, goal: &(i32, i32)) -> u32 {
    dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            NEXT.iter()
                .map(|(xx, yy)| {
                    map.get((y + yy) as usize)
                        .and_then(|r| r.get((x + xx) as usize))
                        .map(|c| ((x + xx, y + yy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == *goal,
    )
    .unwrap()
    .1
}

fn expand_map(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_map: Vec<Vec<u8>> = Vec::new();
    for row in map {
        let mut tmp_row = row.to_vec();
        for k in 1..5 {
            for n in row {
                let value = n + k;
                if value < 10 {
                    tmp_row.push(value);
                } else {
                    tmp_row.push(value - 9);
                }
            }
        }
        new_map.push(tmp_row);
    }
    let mut tmp_map = new_map.to_vec();
    for i in 1..5 {
        for row in &new_map {
            let new_row = row
                .iter()
                .map(|r| if r + i < 10 { r + i } else { r + i - 9 })
                .collect();
            tmp_map.push(new_row);
        }
    }
    new_map = tmp_map.to_vec();
    return new_map;
}
