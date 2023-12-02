use regex::Regex;
fn main() {
    let prefix_pattern = Regex::new(r".*\d+: ").unwrap();
    let input = include_str!("../my_input.txt");
    let rgb: (u32, u32, u32) = (12, 13, 14);
    let res = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let mut invalid = false;
            let cleaned_line = prefix_pattern
                .replace(line, "")
                .parse::<String>()
                .expect("Something went sanitizing the input");
            let rounds = cleaned_line.split(";").collect::<Vec<&str>>();
            for round in rounds {
                round.split(",").for_each(|draw| {
                    let value = Regex::new(r"\d+")
                        .unwrap()
                        .find(&draw)
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    let color = Regex::new(r"\d+\s+").unwrap().replace(draw, "");
                    let trimmed_col = color.trim();
                    match trimmed_col {
                        "red" => {
                            if rgb.0 < value {
                                invalid = true
                            }
                        }
                        "green" => {
                            if rgb.1 < value {
                                invalid = true
                            }
                        }
                        "blue" => {
                            if rgb.2 < value {
                                invalid = true
                            }
                        }
                        _ => (),
                    }
                })
            }
            match invalid {
                false => Some((idx as u32) + 1),
                true => None,
            }
        })
        .sum::<u32>();
    println!("{}", res);
}
