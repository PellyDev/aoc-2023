use regex::Regex;
fn main() {
    let prefix_pattern = Regex::new(r".*\d+: ").unwrap();
    let input = include_str!("../my_input.txt");
    let res = input
        .lines()
        .map(|line| {
            let cleaned_line = prefix_pattern
                .replace(line, "")
                .parse::<String>()
                .expect("Something went wrong sanitizing the input");
            let rounds = cleaned_line.split(";").collect::<Vec<&str>>();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
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
                            red = red.max(value);
                        }
                        "green" => {
                            green = green.max(value);
                        }
                        "blue" => {
                            blue = blue.max(value);
                        }
                        _ => (),
                    }
                });
            }
            red * green * blue
        })
        .sum::<u32>();
    println!("{}", res);
}
