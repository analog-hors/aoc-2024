use std::collections::{HashMap, HashSet};

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

fn trailhead_score(grid: &Grid, endpoints: &mut HashSet<(i32, i32)>, unique_endpoint: bool, x: i32, y: i32) -> u32 {
    match grid.get(&(x, y)) {
        None => 0,
        Some('9') => (endpoints.insert((x, y)) || !unique_endpoint) as u32,
        Some(&c) => {
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter(|&&(nx, ny)| grid.get(&(nx, ny)) == Some(&((c as u8 + 1) as char)))
                .map(|&(nx, ny)| trailhead_score(grid, endpoints, unique_endpoint, nx, ny))
                .sum()
        }
    }
}

fn part_1(input: String) -> u32 {
    let grid = parse_grid(&input);
    grid.iter()
        .filter(|(_, &c)| c == '0')
        .map(|(&(x, y), _)| trailhead_score(&grid, &mut HashSet::new(), true, x, y))
        .sum()
}

fn part_2(input: String) -> u32 {
    let grid = parse_grid(&input);
    grid.iter()
        .filter(|(_, &c)| c == '0')
        .map(|(&(x, y), _)| trailhead_score(&grid, &mut HashSet::new(), false, x, y))
        .sum()
}

aoc::main!();
