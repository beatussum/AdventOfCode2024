fn build(input: &str) -> Vec<Vec<u32>>
{
    input
        .split('\n')
        .filter(|line| !line.is_empty())

        .map(
            |line| {
                line
                    .split_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect()
            }
        )

        .collect()
}

#[cfg(feature = "part_one")]
fn solve(input: &Vec<Vec<u32>>) -> usize
{
    input
        .iter()

        .map(
            |level| {
                let it =
                    level
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|t| t.1)
                        .map(i64::from);

                let it =
                    it
                        .clone()
                        .zip(it.skip(1))
                        .map(|(a, b)| a - b);

                let same_sign =
                    it
                        .clone()
                        .zip(it.clone().skip(1))
                        .all(|(a, b)| (a < 0) == (b < 0));

                let safe_difference =
                    it
                        .map(i64::abs)
                        .all(|a| (a >= 1 && a <= 3));

                same_sign && safe_difference
            }
        )

        .filter(|a| *a)
        .count()
}

#[cfg(feature = "part_two")]
fn solve(input: &Vec<Vec<u32>>) -> usize
{
    input
        .iter()

        .map(
            |level| {
                level
                    .iter()
                    .enumerate()
                    .map(|t| t.0)
                    .map(
                        |to_filter| {
                            let filtered_it =
                                level
                                    .iter()
                                    .copied()
                                    .enumerate()
                                    .filter(|t| t.0 != to_filter)
                                    .map(|t| t.1)
                                    .map(i64::from);

                            let it =
                                filtered_it
                                    .clone()
                                    .zip(filtered_it.skip(1))
                                    .map(|(a, b)| a - b);

                            let same_sign =
                                it
                                    .clone()
                                    .zip(it.clone().skip(1))
                                    .all(|(a, b)| (a < 0) == (b < 0));

                            let safe_difference =
                                it
                                    .map(i64::abs)
                                    .all(|a| (a >= 1 && a <= 3));

                            same_sign && safe_difference
                        }
                    )

                    .any(|a| a)
            }
        )

        .filter(|a| *a)
        .count()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let parsed = build(input.as_str());

    println!("{:?}", solve(&parsed));
}
