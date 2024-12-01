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

    ret.0.sort();
    ret.1.sort();

    ret
}

fn solve(input: &(Vec<u64>, Vec<u64>)) -> u64
{
    input
        .0
        .iter()
        .copied()
        .zip(input.1.iter().copied())
        .map(|(a, b)| (a as i64, b as i64))
        .map(|(a, b)| (a - b).abs() as u64)
        .sum()
}

fn main()
{
    let input = std::fs::read_to_string("input.txt").unwrap();
    let parsed = build(input);
    println!("{:?}", solve(&parsed));
}
