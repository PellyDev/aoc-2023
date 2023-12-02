fn main() {
    let input = include_str!("../my_input.txt");
    let res = input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.next().unwrap_or(0) * 10;
            let last = iter.last().unwrap_or(first / 10);
            first + last
        })
        .sum::<u32>();
    println!("{}", res);
}
