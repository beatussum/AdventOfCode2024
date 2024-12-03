use regex::Regex;
#[cfg(feature = "part_two")] use itertools::Itertools;

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

#[cfg(feature = "part_two")]
fn solve(input: &str) -> u64
{
    let splitter = Regex::new(r"(do\(\)|don't\(\))").unwrap();
    let parser = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    std::iter::once((true, 0))

        .chain(
            splitter
                .captures_iter(input)

                .map(
                    |a| {
                        (
                            a.extract::<1>().1[0] == "do()",
                            a.get(0).unwrap().start()
                        )
                    }
                )
        )

        .chain(std::iter::once((false, input.len())))

    .tuple_windows()
    .into_iter()
    .filter(|((to_do, _), _)| *to_do)
    .map(|((_, begin), (_, end))| (begin, end))
    .map(|(begin, end)| &input[begin..end])

    .flat_map(
        |input| {
            parser
                .captures_iter(input)
                .map(|a| a.extract().1)

                .map(
                    |[a, b]| {
                        [
                            a.parse::<u64>().unwrap(),
                            b.parse::<u64>().unwrap()
                        ]
                    }
                )

                .map(|[a, b]| a * b)
        }
    )

    .sum()
}

fn main()
{
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}
