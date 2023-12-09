use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_nums: Vec<u32>,
    our_nums: Vec<u32>,
}

fn main() {
    let input = include_str!("../my_input.txt");
    let cards: Vec<Card> = input
        .lines()
        .map(|l| {
            let nums = l
                .strip_prefix("Card ")
                .unwrap()
                .trim_start_matches(|c: char| c.is_ascii_digit() || c == ':')
                .trim_start()
                .split("|")
                .map(|slice| {
                    slice
                        .split_ascii_whitespace()
                        .filter_map(|num_as_str| num_as_str.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();
            Card {
                winning_nums: nums[0].clone(),
                our_nums: nums[1].clone(),
            }
        })
        .collect();
    let res = cards
        .iter()
        .map(|card| {
            // the difference between the length of the hash set and the sum of the length of card vectors
            // tells us how many matches we have
            let mut set: HashSet<u32> = HashSet::new();
            set.extend(card.winning_nums.iter());
            set.extend(card.our_nums.iter());
            let hits: i32 = (card.winning_nums.len() + card.our_nums.len() - set.len()) as i32;
            let clamped_hits = (if hits - 1 < 0 { 0 } else { hits - 1 }) as u32;
            if hits > 0 {
                2_u32.pow(clamped_hits)
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("{}", res);
}
