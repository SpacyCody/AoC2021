use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_a(data: &Vec<(String, String)>) {
    let mut outputs: Vec<String> = Vec::new();
    for d in data {
        let all = d.1.split_whitespace();
        for s in all {
            outputs.push(String::from(s))
        }
    }
    let mut nr_unique = 0;
    for output in outputs {
        if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
            nr_unique += 1;
        }
    }
    println!("{}", nr_unique)
}

fn part_b(data: &Vec<(String, String)>) {
    let inputs = vec_of_inputs(data);
    let outputs: Vec<Vec<String>> = vec_of_outputs(data);
    let mut values: Vec<u32> = Vec::new();
    for i in 0..outputs.len() {
        let cipher: [char; 7] = find_cipher(&inputs[i]);
        let mut s: String = String::new();
        for o in &outputs[i] {
            s.push(find_digit(o.to_string(), cipher));
        }
        values.push(s.parse::<u32>().unwrap());
    }
    println!("{}", values.iter().sum::<u32>())
}

fn vec_of_inputs(data: &Vec<(String, String)>) -> Vec<Vec<String>> {
    let mut outputs: Vec<Vec<String>> = Vec::new();
    for d in data {
        let all = d.0.split_whitespace();
        let mut output: Vec<String> = Vec::new();
        for s in all {
            output.push(s.to_string())
        }
        outputs.push(output);
    }
    return outputs;
}

fn vec_of_outputs(data: &Vec<(String, String)>) -> Vec<Vec<String>> {
    let mut outputs: Vec<Vec<String>> = Vec::new();
    for d in data {
        let all = d.1.split_whitespace();
        let mut output: Vec<String> = Vec::new();
        for s in all {
            output.push(s.to_string())
        }
        outputs.push(output);
    }
    return outputs;
}

fn find_cipher(values: &Vec<String>) -> [char; 7] {
    let chars: [char; 7];
    let two_or_five = find_2_or_5(values.to_vec());
    let zero = find_0(values.to_vec(), &two_or_five);
    let one_or_three = find_1_or_3(values.to_vec(), &two_or_five);
    let mut len_five: Vec<String> = Vec::new();
    for string in values {
        if string.len() == 5 {
            len_five.push(string.to_string());
        }
    }
    chars = find_all(len_five, zero, one_or_three, two_or_five);
    return chars;
}

fn find_all(
    len_five: Vec<String>,
    zero: char,
    one_or_three: String,
    two_or_five: String,
) -> [char; 7] {
    let mut chars: [char; 7];
    let mut one: char = 'p';
    let mut two: char = 'p';
    let mut three: char = 'p';
    let mut five: char = 'p';
    let mut six: char = 'p';
    let mut last: String = String::new();
    for string in len_five {
        let in_two_five = if_one_of(&string, &two_or_five);
        let in_one_three = if_one_of(&string, &one_or_three);
        if in_two_five.0 == 1 && in_one_three.0 == 2 {
            for c in string.chars() {
                if six.eq(&'p')
                    && !two_or_five.contains(c)
                    && !one_or_three.contains(c)
                    && c.ne(&zero)
                {
                    six = c;
                }
                if five.eq(&'p') {
                    five = in_two_five.1;
                    let mut two_five: Vec<char> = Vec::new();
                    for tf in two_or_five.chars() {
                        two_five.push(tf);
                    }
                    if five.eq(&two_five[0]) {
                        two = two_five[1]
                    } else {
                        two = two_five[0]
                    }
                }
            }
        } else if in_two_five.0 == 2 && in_one_three.0 == 1 {
            for c in string.chars() {
                if six.eq(&'p')
                    && !two_or_five.contains(c)
                    && !one_or_three.contains(c)
                    && c.ne(&zero)
                {
                    six = c;
                }
                if three.eq(&'p') {
                    three = in_one_three.1;
                    let mut one_three: Vec<char> = Vec::new();
                    for ot in one_or_three.chars() {
                        one_three.push(ot);
                    }
                    if three.eq(&one_three[0]) {
                        one = one_three[1]
                    } else {
                        one = one_three[0]
                    }
                }
            }
        } else {
            last = string;
        }
    }
    chars = [zero, one, two, three, 'p', five, six];
    let four = find_4(chars, last);
    chars[4] = four;
    return chars;
}

fn find_4(chars: [char; 7], string: String) -> char {
    let mut result: char = 'p';
    for s in string.chars() {
        if !chars.contains(&s) {
            result = s;
        }
    }
    return result;
}

fn if_one_of(string: &String, pair: &String) -> (u32, char) {
    let mut nr = 0;
    let mut c = 'p';
    let mut input: Vec<char> = Vec::new();
    for p in pair.chars() {
        input.push(p);
    }
    for cc in string.chars() {
        if cc.eq(&input[0]) {
            nr += 1;
            c = input[0];
        } else if cc.eq(&input[1]) {
            nr += 1;
            c = input[1];
        }
    }
    return (nr, c);
}

fn find_2_or_5(data: Vec<String>) -> String {
    let mut s: String = String::new();
    for d in data {
        if d.len() == 2 {
            s.push_str(&d);
        }
    }
    return s;
}

fn find_0(data: Vec<String>, two_or_five: &String) -> char {
    let mut zero = 'p';
    for string in data {
        if string.len() == 3 {
            for c in string.chars() {
                if !two_or_five.contains(c) {
                    zero = c;
                }
            }
        }
    }
    return zero;
}

fn find_1_or_3(data: Vec<String>, two_or_five: &String) -> String {
    let mut one_or_three: String = String::new();
    for string in data {
        if string.len() == 4 {
            for c in string.chars() {
                if !two_or_five.contains(c) {
                    one_or_three.push(c);
                }
            }
        }
    }
    return one_or_three;
}

fn find_digit(output: String, cipher: [char; 7]) -> char {
    let pattern = [
        ("012456", '0'),
        ("25", '1'),
        ("02346", '2'),
        ("02356", '3'),
        ("1235", '4'),
        ("01356", '5'),
        ("013456", '6'),
        ("025", '7'),
        ("0123456", '8'),
        ("012356", '9'),
    ];
    let mut result: char = 'p';
    let mut values: String = String::new();
    for i in 0..cipher.len() {
        if output.contains(cipher[i]) {
            values.push_str(&i.to_string());
        }
    }
    for p in pattern {
        if values.eq(p.0) {
            result = p.1;
        }
    }
    return result;
}

fn main() -> io::Result<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    let mut data: Vec<(String, String)> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let splited: Vec<&str> = l.split(" | ").collect();
        data.push((String::from(splited[0]), String::from(splited[1])));
    }

    println!("Part A");
    part_a(&data);
    println!("Part B");
    part_b(&data);

    Ok(())
}
