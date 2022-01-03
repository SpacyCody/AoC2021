use std::mem;

fn part_a(input: &Vec<u8>, rules: &Vec<([u8; 2], [u8; 2])>) {
    let (min, max) = run(input, rules, 10);
    println!("{}", max - min)
}

fn part_b(input: &Vec<u8>, rules: &Vec<([u8; 2], [u8; 2])>) {
    let (min, max) = run(input, rules, 40);
    println!("{}", max - min)
}

fn main() {
    let (input, rules) = include_str!("../data.txt").split_once("\n\n").unwrap();
    let input = input.as_bytes().to_vec();
    let rules = rules
        .lines()
        .map(|l| {
            let (s, c) = l.split_once(" -> ").unwrap();
            let (s,c) = (s.as_bytes(), c.as_bytes()[0]);
            ([s[0], s[1]], [s[0], c])
        })
        .collect::<Vec<_>>();
    println!("Part A");
    part_a(&input, &rules);
    println!("Part B");
    part_b(&input, &rules);
}

fn run(input: &Vec<u8>, rules: &Vec<([u8; 2], [u8; 2])>, runs: u32) -> (u64, u64) {
    let mut rules = rules.to_vec();
    rules.sort_unstable_by_key(|r| r.0);

    let rule = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                rules.binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    
    let (mut num, mut next) = (vec![0; rule.len()], vec![0; rule.len()]);
    input.windows(2).for_each(|key| num[rule.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);
    (0..runs).for_each(|_| {
        num.iter_mut().zip(&rule).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occurrences = [0; (b'Z' - b'A') as usize];
    occurrences[(input.last().unwrap() - b'A') as usize] += 1;
    rule.iter()
        .zip(num)
        .for_each(|(r, n)| occurrences[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occurrences
        .iter()
        .filter(|&&n| n != 0)
        .fold((u64::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));
    return (min, max);
}
