fn parse_input(input: String) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn report_is_safe(report: &[u32]) -> bool {
    let ordered = report.is_sorted() || report.is_sorted_by_key(std::cmp::Reverse);
    let small_steps = report.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));
    ordered && small_steps
}

fn report_is_tolerable(report: &[u32]) -> bool {
    (0..report.len()).any(|i| {
        let mut edited = report.to_vec();
        edited.remove(i);
        report_is_safe(&edited)
    })
}

fn part_1(input: String) -> usize {
    parse_input(input).iter().filter(|r| report_is_safe(r)).count()
}

fn part_2(input: String) -> usize {
    parse_input(input).iter().filter(|r| report_is_tolerable(r)).count()
}

aoc::main!();
