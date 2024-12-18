use std::collections::{HashSet, VecDeque};

fn parse_cell(cell: &str) -> (i32, i32) {
    let (x, y) = cell.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn shortest_path_to_end(walls: &HashSet<(i32, i32)>, size: i32) -> Option<u32> {
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    to_visit.push_back(((0, 0), 0));
    visited.insert((0, 0));
    while let Some(((x, y), cost)) = to_visit.pop_front() {
        if (x, y) == (size - 1, size - 1) {
            return Some(cost);
        }
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if nx < 0 || nx >= size || ny < 0 || ny >= size {
                continue;
            }
            if walls.contains(&(nx, ny)) {
                continue;
            }
            if visited.insert((nx, ny)) {
                to_visit.push_back(((nx, ny), cost + 1));
            }
        }
    }

    None
}

fn part_1(input: String) -> u32 {
    let walls = input.lines().map(parse_cell).take(1024).collect();
    shortest_path_to_end(&walls, 71).unwrap()
}

fn part_2(input: String) -> String {
    let mut walls = HashSet::new();
    for (x, y) in input.lines().map(parse_cell) {
        walls.insert((x, y));
        if shortest_path_to_end(&walls, 71).is_none() {
            return format!("{},{}", x, y);
        }
    }
    panic!("no solution")
}

aoc::main!();
