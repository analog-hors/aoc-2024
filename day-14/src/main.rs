type Vec2 = (i64, i64);

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const SIZE: Vec2 = (WIDTH as i64, HEIGHT as i64);
const CHRISTMAS_TREE: &[&[u8]] = &[
    b"###############################",
    b"#.............................#",
    b"#.............................#",
    b"#.............................#",
    b"#.............................#",
    b"#..............#..............#",
    b"#.............###.............#",
    b"#............#####............#",
    b"#...........#######...........#",
    b"#..........#########..........#",
    b"#............#####............#",
    b"#...........#######...........#",
    b"#..........#########..........#",
    b"#.........###########.........#",
    b"#........#############........#",
    b"#..........#########..........#",
    b"#.........###########.........#",
    b"#........#############........#",
    b"#.......###############.......#",
    b"#......#################......#",
    b"#........#############........#",
    b"#.......###############.......#",
    b"#......#################......#",
    b"#.....###################.....#",
    b"#....#####################....#",
    b"#.............###.............#",
    b"#.............###.............#",
    b"#.............###.............#",
    b"#.............................#",
    b"#.............................#",
    b"#.............................#",
    b"#.............................#",
    b"###############################",
];

fn parse_vec(vec: &str) -> Vec2 {
    let (x, y) = vec.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse_input(input: &str) -> Vec<(Vec2, Vec2)> {
    input.lines()
        .map(|robot| {
            let (pos, vel) = robot.split_once(' ').unwrap();
            (parse_vec(&pos[2..]), parse_vec(&vel[2..]))
        })
        .collect()
}

fn final_position((x, y): Vec2, (dx, dy): Vec2, (w, h): Vec2, steps: i64) -> Vec2 {
    ((x + dx * steps).rem_euclid(w), (y + dy * steps).rem_euclid(h))
}

fn quadrant_index((x, y): Vec2, (w, h): Vec2) -> Option<usize> {
    let midpoint = x * 2 + 1 == w || y * 2 + 1 == h;
    let index = (y >= h / 2) as usize * 2 + (x >= w / 2) as usize;
    (!midpoint).then_some(index)
}

fn final_grid(robots: &[(Vec2, Vec2)], steps: i64) -> [[u32; WIDTH]; HEIGHT] {
    let mut grid = [[0; WIDTH]; HEIGHT];
    for &(pos, vel) in robots {
        let (ex, ey) = final_position(pos, vel, SIZE, steps);
        grid[ey as usize][ex as usize] += 1;
    }
    grid
}

fn grid_contains_template(grid: &[[u32; WIDTH]; HEIGHT], template: &[&[u8]]) -> bool {
    let test_pos = |sx: usize, sy: usize| {
        for (ty, row) in template.iter().enumerate() {
            for (tx, &cell) in row.iter().enumerate() {
                if sx + tx >= WIDTH || sy + ty >= HEIGHT {
                    return false;
                }
                if (grid[sy + ty][sx + tx] != 0) != (cell != b'.') {
                    return false;
                }
            }
        }
        true
    };
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if test_pos(x, y) {
                return true;
            }
        }
    }
    false
}

fn part_1(input: String) -> u32 {
    let robots = parse_input(&input);
    let mut occupancy = [0; 4];
    for (pos, vel) in robots {
        let end = final_position(pos, vel, SIZE, 100);
        if let Some(quadrant) = quadrant_index(end, SIZE) {
            occupancy[quadrant] += 1;
        }
    }
    occupancy.iter().product()
}

fn part_2(input: String) -> i64 {
    let robots = parse_input(&input);
    (0..)
        .find(|&steps| {
            let grid = final_grid(&robots, steps);
            grid_contains_template(&grid, CHRISTMAS_TREE)
        })
        .unwrap()
}

aoc::main!();
