type Vec2 = (i64, i64);

fn parse_vec(vec: &str) -> Vec2 {
    let (x, y) = vec.split_once(", ").unwrap();
    (x[2..].parse().unwrap(), y[2..].parse().unwrap())
}

fn parse_input(input: &str) -> Vec<(Vec2, Vec2, Vec2)> {
    let mut machines = Vec::new();

    let mut basis_i = (0, 0);
    let mut basis_j = (0, 0);
    for line in input.lines() {
        match line.split_once(": ") {
            Some(("Button A", basis)) => basis_i = parse_vec(basis),
            Some(("Button B", basis)) => basis_j = parse_vec(basis),
            Some(("Prize", target)) => {
                let target = parse_vec(target);
                machines.push((basis_i, basis_j, target));
            }
            _ => {}
        }
    }

    machines
}

fn solve_matrix((ix, iy): Vec2, (jx, jy): Vec2, (px, py): Vec2) -> Option<Vec2> {
    let det_x = px * jy - jx * py;
    let det_y = ix * py - px * iy;
    let det_d = ix * jy - jx * iy;
    (det_x % det_d == 0 && det_y % det_d == 0).then_some((det_x / det_d, det_y / det_d))
}

fn part_1(input: String) -> i64 {
    parse_input(&input).iter()
        .filter_map(|&(i, j, p)| solve_matrix(i, j, p))
        .map(|(x, y)| x * 3 + y)
        .sum()
}

fn part_2(input: String) -> i64 {
    parse_input(&input).iter()
        .filter_map(|&(i, j, (px, py))| solve_matrix(i, j, (px + 10_000_000_000_000, py + 10_000_000_000_000)))
        .map(|(x, y)| x * 3 + y)
        .sum()
}

aoc::main!();
