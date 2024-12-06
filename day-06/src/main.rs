use std::collections::{HashMap, HashSet};

type Grid = HashMap<(i32, i32), char>;
type GuardState = ((i32, i32), (i32, i32));

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

fn find_and_remove_guard(grid: &mut Grid) -> (i32, i32) {
    let (&guard, _) = grid.iter().find(|(_, &c)| c == '^').unwrap();
    *grid.get_mut(&guard).unwrap() = '.';
    guard
}

fn simulate_guard(grid: &Grid, (mut x, mut y): (i32, i32)) -> impl Iterator<Item=GuardState> + '_ {
    let mut dx = 0;
    let mut dy = -1;
    std::iter::from_fn(move || loop {
        match grid.get(&(x, y)) {
            Some('#') => {
                x -= dx;
                y -= dy;
                (dx, dy) = (-dy, dx);
            }
            Some(_) => {
                x += dx;
                y += dy;
                return Some(((x - dx, y - dy), (dx, dy)));
            }
            None => return None,
        }
    })
}

fn unique_guard_locations(grid: &Grid, guard: (i32, i32)) -> HashSet<(i32, i32)> {
    simulate_guard(grid, guard).map(|(c, _)| c).collect()
}

fn guard_loops(grid: &Grid, guard: (i32, i32)) -> bool {
    let mut states = HashSet::new();
    for state in simulate_guard(grid, guard) {
        if !states.insert(state) {
            return true;
        }
    }
    false
}

fn part_1(input: String) -> usize {
    let mut grid = parse_grid(&input);
    let guard = find_and_remove_guard(&mut grid);
    unique_guard_locations(&grid, guard).len()
}

fn part_2(input: String) -> u32 {
    let mut grid = parse_grid(&input);
    let guard = find_and_remove_guard(&mut grid);
    
    let mut solutions = 0;
    for cell in unique_guard_locations(&grid, guard) {
        *grid.get_mut(&cell).unwrap() = '#';
        if guard_loops(&grid, guard) {
            solutions += 1;
        }
        *grid.get_mut(&cell).unwrap() = '.';
    }
    solutions
}

aoc::main!();
