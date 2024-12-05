use std::collections::HashMap;

type Grid = HashMap<(i32, i32), char>;

fn parse_grid(grid: &str) -> Grid {
    grid.lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

fn matches_xmas(grid: &Grid, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    "XMAS".chars()
        .enumerate()
        .map(|(i, c)| ((x + dx * i as i32, y + dy * i as i32), c))
        .all(|(cell, c)| grid.get(&cell) == Some(&c))
}

fn matches_x_mas(grid: &Grid, x: i32, y: i32) -> bool {
    if grid.get(&(x, y)) != Some(&'A') {
        return false;
    }

    let tl = grid.get(&(x - 1, y - 1));
    let br = grid.get(&(x + 1, y + 1));
    if tl != Some(&'M') && br != Some(&'M') {
        return false;
    }
    if tl != Some(&'S') && br != Some(&'S') {
        return false;
    }

    let tr = grid.get(&(x + 1, y - 1));
    let bl = grid.get(&(x - 1, y + 1));
    if tr != Some(&'M') && bl != Some(&'M') {
        return false;
    }
    if tr != Some(&'S') && bl != Some(&'S') {
        return false;
    }

    true
}

const DELTAS: [(i32, i32); 8] = [
    (1, 0), (0, -1), (-1, 0), (0, 1),
    (1, 1), (1, -1), (-1, -1), (-1, 1),
];

fn part_1(input: String) -> usize {
    let grid = parse_grid(&input);
    grid.keys()
        .map(|&(x, y)| {
            DELTAS.iter()
                .filter(|&&(dx, dy)| matches_xmas(&grid, x, y, dx, dy))
                .count()
        })
        .sum()
}

fn part_2(input: String) -> usize {
    let grid = parse_grid(&input);
    grid.keys()
        .filter(|&&(x, y)| matches_x_mas(&grid, x, y))
        .count()
}

aoc::main!();
