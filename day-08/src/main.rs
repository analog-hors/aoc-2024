use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut width = 0;
    let mut height = 0;
    let mut antennas = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            width = width.max(x as i32 + 1);
            height = height.max(y as i32 + 1);
            if c != '.' {
                antennas.entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }
    (antennas, (width, height))
}

fn iter_antenna_pairs(antennas: &[Point]) -> impl Iterator<Item=(Point, Point)> + '_ {
    antennas.iter()
        .flat_map(|&a| {
            antennas.iter()
                .map(move |&b| (a, b))
                .filter(|(a, b)| a != b)
        })
}

fn part_1(input: String) -> usize {
    let (antennas, (width, height)) = parse_input(&input);
    antennas.iter()
        .flat_map(|(_, antennas)| {
            iter_antenna_pairs(antennas)
                .map(|((ax, ay), (bx, by))| (ax + (bx - ax) * 2, ay + (by - ay) * 2))
                .filter(|(x, y)| (0..width).contains(x) && (0..height).contains(y))
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part_2(input: String) -> usize {
    let (antennas, (width, height)) = parse_input(&input);
    
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for ((ax, ay), (bx, by)) in iter_antenna_pairs(&antennas) {
            let dx = bx - ax;
            let dy = by - ay;
            let mut x = ax + dx;
            let mut y = ay + dy;
            while (0..width).contains(&x) && (0..height).contains(&y) {
                antinodes.insert((x, y));
                x += dx;
                y += dy;
            }
        }
    }

    antinodes.len()
}

aoc::main!();
