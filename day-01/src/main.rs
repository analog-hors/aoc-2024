fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut locs = Vec::new();
    let mut sims = Vec::new();
    for line in input.lines() {
        let (loc, sim) = line.split_once("   ").unwrap();
        locs.push(loc.parse().unwrap());
        sims.push(sim.parse().unwrap());
    }
    
    (locs, sims)
}

fn part_1(input: String) -> u32 {
    let (mut locs, mut sims) = parse_input(input);
    locs.sort_unstable();
    sims.sort_unstable();
    locs.iter().zip(&sims).map(|(&l, &s)| l.abs_diff(s)).sum()
}

fn part_2(input: String) -> u32 {
    let (locs, sims) = parse_input(input);
    let mut score = 0;
    for &loc in &locs {
        for &sim in &sims {
            if loc == sim {
                score += loc;
            }
        }
    }
    score
}

aoc::main!();
