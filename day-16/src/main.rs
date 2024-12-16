use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

type Node = ((i32, i32), (i32, i32));
type Grid = HashMap<(i32, i32), char>;
type Costs = HashMap<Node, u32>;

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

fn find_marker(grid: &Grid, marker: char) -> (i32, i32) {
    grid.iter().find(|(_, &c)| c == marker).map(|(&n, _)| n).unwrap()
}

fn visit_neighbours(grid: &Grid, ((x, y), (dx, dy)): Node, sign: i32, visitor: &mut impl FnMut(Node, u32)) {
    let next = (x + dx * sign, y + dy * sign);
    if grid.get(&next).unwrap_or(&'#') != &'#' {
        visitor((next, (dx, dy)), 1);
    }
    visitor(((x, y), (-dy, dx)), 1000);
    visitor(((x, y), (dy, -dx)), 1000);
}

fn node_costs_to_end(grid: &Grid, end: (i32, i32)) -> Costs {
    let mut costs = Costs::new();
    let mut to_visit = BinaryHeap::new();
    
    for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        costs.insert((end, delta), 0);
        to_visit.push((Reverse(0), (end, delta)));
    }

    while let Some((Reverse(cost), node)) = to_visit.pop() {
        visit_neighbours(grid, node, -1, &mut |neighbour, weight| {
            let n_cost = cost + weight;
            let old_cost = costs.entry(neighbour).or_insert(u32::MAX);
            if n_cost < *old_cost {
                *old_cost = n_cost;
                to_visit.push((Reverse(n_cost), neighbour));
            }
        });
    }

    costs
}

fn shortest_paths(grid: &Grid, costs: &Costs, path: &mut Vec<Node>, node: Node, visitor: &mut impl FnMut(&[Node])) {
    path.push(node);

    let cost = *costs.get(&node).unwrap();
    if cost == 0 {
        visitor(path);
    } else {
        visit_neighbours(grid, node, 1, &mut |neighbour, weight| {
            if *costs.get(&neighbour).unwrap() + weight == cost {
                shortest_paths(grid, costs, path, neighbour, visitor);
            }
        });
    }

    path.pop();
}

fn part_1(input: String) -> u32 {
    let grid = parse_grid(&input);
    let start = find_marker(&grid, 'S');
    let end = find_marker(&grid, 'E');
    
    let costs = node_costs_to_end(&grid, end);
    *costs.get(&(start, (1, 0))).unwrap()
}

fn part_2(input: String) -> usize {
    let grid = parse_grid(&input);
    let start = find_marker(&grid, 'S');
    let end = find_marker(&grid, 'E');
    
    let costs = node_costs_to_end(&grid, end);
    let mut best_path_tiles = HashSet::new();
    shortest_paths(&grid, &costs, &mut Vec::new(), (start, (1, 0)), &mut |path| {
        best_path_tiles.extend(path.iter().map(|&(cell, _)| cell));
    });

    best_path_tiles.len()
}

aoc::main!();