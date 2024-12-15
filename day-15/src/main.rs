use std::collections::HashSet;

type Cells = HashSet<(i32, i32)>;
type Warehouse = (Cells, Cells, (i32, i32));

fn parse_warehouse(input: &str, wide: bool) -> Warehouse {
    let mut walls = Cells::new();
    let mut boxes = Cells::new();
    let mut robot = (0, 0);
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let cell = match wide {
                false => (x as i32, y as i32),
                true => (x as i32 * 2, y as i32),
            };
            match c {
                '#' => {
                    walls.insert(cell);
                }
                'O' => {
                    boxes.insert(cell);
                }
                '@' => {
                    robot = cell;
                }
                _ => {}
            }
        }
    }
    (walls, boxes, robot)
}

fn parse_moves(moves: &str) -> Vec<char> {
    moves.lines().flat_map(str::chars).collect()
}

fn parse_input(input: &str, wide: bool) -> (Warehouse, Vec<char>) {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();
    (parse_warehouse(warehouse, wide), parse_moves(moves))
}

fn move_delta(mv: char) -> (i32, i32) {
    match mv {
        '^' => ( 0, -1),
        '>' => ( 1,  0),
        'v' => ( 0,  1),
        '<' => (-1,  0),
        _ => panic!(),
    }
}

fn point_collides(colliders: &Cells, (x, y): (i32, i32), wide: bool) -> Option<(i32, i32)> {
    match wide {
        false => colliders.get(&(x, y)).copied(),
        true => colliders.get(&(x, y)).or(colliders.get(&(x - 1, y))).copied(),
    }
}

fn collider_points((x, y): (i32, i32), wide: bool) -> impl Iterator<Item=(i32, i32)> {
    let width = 1 + wide as i32;
    (x..x + width).map(move |x| (x, y))
}

fn collect_linked(colliders: &Cells, linked: &mut Cells, (x, y): (i32, i32), (dx, dy): (i32, i32), wide: bool) {
    if let Some((cx, cy)) = point_collides(colliders, (x + dx, y + dy), wide) {
        if linked.insert((cx, cy)) {
            for (px, py) in collider_points((cx, cy), wide) {
                collect_linked(colliders, linked, (px, py), (dx, dy), wide);
            }
        }
    }    
}

fn apply_moves(walls: &Cells, boxes: &mut Cells, (rx, ry): &mut (i32, i32), moves: &[char], wide: bool) {
    let mut colliders = walls.union(boxes).copied().collect();
    for &mv in moves {
        let (dx, dy) = move_delta(mv);
        let mut linked = Cells::new();
        collect_linked(&colliders, &mut linked, (*rx, *ry), (dx, dy), wide);
        if linked.is_disjoint(walls) {
            for &(bx, by) in &linked {
                boxes.remove(&(bx, by));
                colliders.remove(&(bx, by));
            }
            for &(bx, by) in &linked {
                boxes.insert((bx + dx, by + dy));
                colliders.insert((bx + dx, by + dy));
            }
            *rx += dx;
            *ry += dy;
        }
    }
}

fn gps_sum(boxes: &Cells) -> i32 {
    boxes.iter().map(|(x, y)| y * 100 + x).sum()
}

fn part_1(input: String) -> i32 {
    let ((walls, mut boxes, mut robot), moves) = parse_input(&input, false);
    apply_moves(&walls, &mut boxes, &mut robot, &moves, false);
    gps_sum(&boxes)
}

fn part_2(input: String) -> i32 {
    let ((walls, mut boxes, mut robot), moves) = parse_input(&input, true);
    apply_moves(&walls, &mut boxes, &mut robot, &moves, true);
    gps_sum(&boxes)
}

aoc::main!();
