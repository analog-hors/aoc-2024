use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u64> {
    input.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn split_if_even_digits(n: u64) -> Option<(u64, u64)> {
    let mut f = 1;
    while f * f <= n {
        f *= 10;
    }
    
    if f * f > n * 10 {
        return None;
    }

    Some((n / f, n % f))
}

fn stones_after(init: u64, blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    let result = if blinks == 0 {
        1
    } else if let Some(&cached) = cache.get(&(init, blinks)) {
        cached
    } else if init == 0 {
        stones_after(1, blinks - 1, cache)
    } else if let Some((left, right)) = split_if_even_digits(init) {
        stones_after(left, blinks - 1, cache) + stones_after(right, blinks - 1, cache)
    } else {
        stones_after(init * 2024, blinks - 1, cache)
    };
    
    cache.insert((init, blinks), result);
    result
}

fn part_1(input: String) -> u64 {
    let mut cache = HashMap::new();
    parse_input(&input).iter().map(|&n| stones_after(n, 25, &mut cache)).sum()
}

fn part_2(input: String) -> u64 {
    let mut cache = HashMap::new();
    parse_input(&input).iter().map(|&n| stones_after(n, 75, &mut cache)).sum()
}

aoc::main!();
