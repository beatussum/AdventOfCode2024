use regex::Regex;

#[cfg(feature = "part_one")]
fn solve(input: &str) -> u64
{
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|c| c.extract::<2>().1.map(str::parse::<u64>).map(Result::unwrap))
        .map(|a| a[0] * a[1])
        .sum()
}

fn main()
{
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}
