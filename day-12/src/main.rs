use std::collections::{HashMap, HashSet};

type Grid = HashMap<(i32, i32), char>;
type Region = HashSet<(i32, i32)>;

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

fn neighbours(x: i32, y: i32) -> [(i32, i32); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn visit_region(grid: &Grid, region: &mut Region, x: i32, y: i32) {
    if let Some(c) = grid.get(&(x, y)) {
        region.insert((x, y));
        for (nx, ny) in neighbours(x, y) {
            if !region.contains(&(nx, ny)) && grid.get(&(nx, ny)) == Some(c) {
                visit_region(grid, region, nx, ny);
            }
        }
    }
}

fn grid_regions(grid: &Grid) -> Vec<Region> {
    let mut visited = Region::new();
    let mut regions = Vec::new();
    for &(x, y) in grid.keys() {
        if !visited.contains(&(x, y)) {
            let mut region = Region::new();
            visit_region(grid, &mut region, x, y);
            visited.extend(&region);
            regions.push(region);
        }
    }
    regions
}

fn region_perimeter(region: &Region) -> usize {
    region.iter()
        .flat_map(|&(x, y)| neighbours(x, y))
        .filter(|&(x, y)| !region.contains(&(x, y)))
        .count()
}

fn region_sides(region: &Region) -> usize {
    let mut corners = 0;
    for &(x, y) in region {
        for (dx, dy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let horizontal = region.contains(&(x + dx, y));
            let vertical = region.contains(&(x, y + dy));
            let across = region.contains(&(x + dx, y + dy));
            if !horizontal && !vertical || horizontal && vertical && !across {
                corners += 1;
            }
        }
    }
    corners
}

fn part_1(input: String) -> usize {
    let grid = parse_grid(&input);
    grid_regions(&grid).iter()
        .map(|r| region_perimeter(r) * r.len())
        .sum()
}

fn part_2(input: String) -> usize {
    let grid = parse_grid(&input);
    grid_regions(&grid).iter()
        .map(|r| region_sides(r) * r.len())
        .sum()
}

aoc::main!();
