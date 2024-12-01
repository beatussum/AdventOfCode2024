#[cfg(feature = "part_two")] use std::collections::HashMap;

fn build(input: String) -> (Vec<u64>, Vec<u64>)
{
    let mut ret = (Vec::new(), Vec::new());

    for line in input.split('\n') {
        if !line.is_empty() {
            let mut it = line.split_whitespace();

            ret.0.push(it.next().unwrap().parse().unwrap());
            ret.1.push(it.next().unwrap().parse().unwrap());
        }
    }

    ret
}

#[cfg(feature = "part_one")]
fn solve(input: &mut (Vec<u64>, Vec<u64>)) -> u64
{
    input.0.sort();
    input.1.sort();

    input
        .0
        .iter()
        .copied()
        .zip(input.1.iter().copied())
        .map(|(a, b)| (a as i64, b as i64))
        .map(|(a, b)| (a - b).abs() as u64)
        .sum()
}

#[cfg(feature = "part_two")]
fn solve(input: &(Vec<u64>, Vec<u64>)) -> u64
{
    let mut left = HashMap::<u64, u64>::new();
    let mut right = HashMap::<u64, u64>::new();

    for (a, b) in input.0.iter().copied().zip(input.1.iter().copied()) {
        let left_value = *left.entry(a).or_insert(0) + 1;
        let right_value = *right.entry(b).or_insert(0) + 1;

        left.insert(a, left_value);
        right.insert(b, right_value);
    }

    input
        .0
        .iter()
        .copied()
        .map(|a| a * right.get(&a).unwrap_or(&0))
        .sum()
}

fn main()
{
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut parsed = build(input);
    println!("{:?}", solve(&mut parsed));
}
