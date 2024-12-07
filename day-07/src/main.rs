fn parse_input(input: &str) -> impl Iterator<Item=(u64, Vec<u64>)> + '_ {
    input.lines()
        .map(|line| {
            let (target, terms) = line.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let terms = terms.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (target, terms)
        })
}

fn possibly_true(target: u64, terms: &[u64], allow_cat: bool) -> bool {
    fn cat(n: u64, m: u64) -> u64 {
        let mut f = 1;
        while f <= m {
            f *= 10;
        }
        n * f + m
    }

    fn visit(target: u64, n: u64, terms: &[u64], allow_cat: bool) -> bool {
        match terms.split_first() {
            Some((m, rest)) => {
                visit(target, n + m, rest, allow_cat)
                    || visit(target, n * m, rest, allow_cat)
                    || allow_cat && visit(target, cat(n, *m), rest, allow_cat)
            }
            None => n == target,
        }
    }

    match terms.split_first() {
        Some((n, rest)) => visit(target, *n, rest, allow_cat),
        None => false,
    }
}

fn part_1(input: String) -> u64 {
    parse_input(&input)
        .filter(|(target, terms)| possibly_true(*target, terms, false))
        .map(|(target, _)| target)
        .sum()
}

fn part_2(input: String) -> u64 {
    parse_input(&input)
        .filter(|(target, terms)| possibly_true(*target, terms, true))
        .map(|(target, _)| target)
        .sum()
}

aoc::main!();
