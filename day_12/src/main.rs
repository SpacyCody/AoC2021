use linked_hash_map::LinkedHashMap;
use std::collections::{BTreeMap, HashMap};

fn part_a(map: &CaveMap){
    let paths = map.count_paths(
        |cave, path| cave.chars().all(|c| c.is_uppercase()) || !path.contains_key(cave),
        |cave| cave == "end",
    );
    println!("{}", paths)
}
fn part_b(map: &CaveMap){
    let paths = map.count_paths(
        |cave, path| {
            cave != "start"
                && (cave.chars().all(|c| c.is_uppercase())
                    || path.values().all(|x| *x < 2)
                    || !path.contains_key(cave))
        },
        |cave| cave == "end",
    );
    println!("{}", paths)
}

fn main(){
    let data = include_str!("../data.txt");

    let map: CaveMap = CaveMap::from_data(data);
    println!("Part A");
    part_a(&map);
    println!("Part B");
    part_b(&map);
}

struct CaveMap {
    connections: HashMap<String, Vec<String>>,
}

impl CaveMap {
    fn from_data(data: &str) -> Self {
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();

        for (a, b) in data
            .lines()
            .map(|l| l.split_once("-").unwrap())
        {
            connections
                .entry(a.to_string())
                .or_default()
                .push(b.to_string());
            connections
                .entry(b.to_string())
                .or_default()
                .push(a.to_string());
        }

        CaveMap { connections }
    }

    fn count_paths<Pv, Pf>(&self, valid_move: Pv, final_move: Pf) -> usize
    where
        Pv: Fn(&String, &BTreeMap<String, u8>) -> bool,
        Pf: Fn(&String) -> bool,
    {
        let mut path_count = 0;
        let mut paths = LinkedHashMap::new();
        paths.insert(
            (
                BTreeMap::from([("start".to_string(), 1u8)]),
                "start".to_string(),
            ),
            1usize,
        );

        while !paths.is_empty() {
            let (current_path, count) = paths.pop_front().unwrap();
            for cave in &self.connections[&current_path.1] {
                if final_move(cave) {
                    path_count += count;
                } else if valid_move(cave, &current_path.0) {
                    let mut continued_path = current_path.clone();
                    if cave.chars().all(|c| c.is_lowercase()) {
                        *continued_path.0.entry(cave.to_string()).or_default() += 1;
                    }
                    continued_path.1 = cave.clone();
                    if paths.contains_key(&continued_path) {
                        paths[&continued_path] += count;
                    } else {
                        paths.insert(continued_path, count);
                    }
                }
            }
        }

        path_count
    }
}