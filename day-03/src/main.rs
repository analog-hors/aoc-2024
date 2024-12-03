fn parse_mul_args(s: &str) -> Option<(u32, u32)> {
    let (a, b) = s[..s.find(')')?].split_once(',')?;
    Some((a.parse().ok()?, b.parse().ok()?))
}

fn mul_sum(s: &str) -> u32 {
    s.match_indices("mul(")
        .filter_map(|(i, _)| parse_mul_args(&s[i + 4..]))
        .map(|(a, b)| a * b)
        .sum()
}

fn part_1(input: String) -> u32 {
    mul_sum(&input)
}

fn part_2(input: String) -> u32 {
    input.split("do()")
        .map(|s| mul_sum(s.split("don't()").next().unwrap()))
        .sum()
}

aoc::main!();
