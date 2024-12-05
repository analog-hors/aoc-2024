use std::collections::{HashMap, HashSet};

type RuleGraph = HashMap<u32, (HashSet<u32>, HashSet<u32>)>;

fn parse_rules(rules: &str) -> Vec<(u32, u32)> {
    rules.lines()
        .map(|rule| {
            let (a, b) = rule.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates.lines()
        .map(|update| {
            update.split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    (parse_rules(rules), parse_updates(updates))
}

fn make_rule_graph(rules: &[(u32, u32)]) -> RuleGraph {
    let mut graph = RuleGraph::new();
    for &(a, b) in rules {
        graph.entry(a).or_default().1.insert(b);
        graph.entry(b).or_default().0.insert(a);
    }
    graph
}

fn extract_graph_subset(graph: &RuleGraph, nodes: &[u32]) -> RuleGraph {
    let mut subset = RuleGraph::new();
    for &node in nodes {
        if let Some((lesser, greater)) = graph.get(&node) {
            let lesser = lesser.iter().copied().filter(|n| nodes.contains(n)).collect();
            let greater = greater.iter().copied().filter(|n| nodes.contains(n)).collect();
            subset.insert(node, (lesser, greater));
        }
    }
    subset
}

fn find_lowest_node(graph: &RuleGraph) -> Option<u32> {
    let mut node = *graph.keys().next()?;
    loop {
        let (lesser, _) = graph.get(&node)?;
        match lesser.iter().next() {
            Some(&next) => node = next,
            None => return Some(node),
        }
    }
}

fn remove_node(graph: &mut RuleGraph, node: u32) {
    if let Some((lesser, greater)) = graph.remove(&node) {
        for neighbour in lesser {
            graph.get_mut(&neighbour).unwrap().1.remove(&node);
        }
        for neighbour in greater {
            graph.get_mut(&neighbour).unwrap().0.remove(&node);
        }
    }
}

fn sort_graph(mut graph: RuleGraph) -> Vec<u32> {
    let mut sorted = Vec::new();
    while let Some(node) = find_lowest_node(&graph) {
        remove_node(&mut graph, node);
        sorted.push(node);
    }
    sorted
}

fn part_1(input: String) -> u32 {
    let (rules, updates) = parse_input(&input);
    let graph = make_rule_graph(&rules);
    updates.iter()
        .filter(|&update| {
            let subset = extract_graph_subset(&graph, update);
            let sorted = sort_graph(subset);
            &sorted == update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2(input: String) -> u32 {
    let (rules, updates) = parse_input(&input);
    let graph = make_rule_graph(&rules);
    updates.iter()
        .filter_map(|update| {
            let subset = extract_graph_subset(&graph, update);
            let sorted = sort_graph(subset);
            (&sorted != update).then_some(sorted)
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

aoc::main!();
