use std::collections::HashMap;

fn process_line(str: &str, map: &HashMap<&str, char>) -> String {
    let mut out: Vec<char> = Vec::new();
    let window_size: usize = if str.len() < 5 { str.len() } else { 5 };
    let str_as_vec = str.chars().collect::<Vec<char>>();
    for window in str_as_vec.windows(window_size) {
        for (i, char_iw) in window.iter().enumerate() {
            if char_iw.is_numeric() {
                out.push(*char_iw);
                continue;
            };
            let mut buf = Vec::new();
            for j in i..window.len() {
                buf.push(*window.get(j).unwrap());
                match map.get(String::from_iter(buf.iter()).as_str()) {
                    Some(val) => {
                        out.push(*val);
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }
    String::from_iter(out.iter())
}
fn main() {
    let map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let input = include_str!("../sample_input.txt");
    let res = input
        .lines()
        .map(|line| {
            let line = process_line(line, &map);
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter
                .next()
                .expect(format!("No number found in line '{}'", line).as_str())
                * 10;
            let last = iter.last().unwrap_or(first / 10);
            first + last
        })
        .sum::<u32>();
    println!("{}", res);
}
