use std::cmp::max;
use std::collections::HashMap;

use petgraph::Graph;
use petgraph::Undirected;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use std::io;
use std::io::Read;
use itertools::Itertools;

#[derive(Debug)]
struct Node{flow: usize, starting: bool}
impl From<(String, usize, bool)> for Node {
    fn from(value: (String, usize, bool)) -> Self {
        Self{flow: value.1, starting: value.2}
    }
}
type N = Node;
type E = usize;

struct GraphDetails<'a> {
    graph: &'a Graph<N, E, Undirected>,
    distances: &'a HashMap<(NodeIndex, NodeIndex), usize>,
    position: NodeIndex,
    remaining: Vec<NodeIndex>,
    time: usize,
    pressure: usize,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut graph = parse_input(&buf);
    reduce_graph(&mut graph);
    let max_pressure = max_pressure_solo(&graph, 30);
    println!("Most pressure you can release: {max_pressure}");
    let max_pressure = max_pressure_with_elephant(&graph, 26);
    println!("Most pressure you and an elephant can release: {max_pressure}");
}

fn parse_input(input: &str) -> Graph<N, E, Undirected> {
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();
    let mut edges = vec![];
    for line in input.lines() {
        let mut split = line.trim().split([';', ' ', '=']);
        let label = split.nth(1).unwrap().to_string();
        let rate = split.nth(3).unwrap().parse::<usize>().unwrap();
        let index = graph.add_node((label.clone(), rate, label == "AA").into());
        nodes.insert(label.clone(), index);
        let neighbours = split.skip(5).map(|s| s.replace(',', "")).collect::<Vec<String>>();
        edges.append(&mut neighbours.iter().map(|n| (label.clone(), n.clone())).collect());
    }
    for (n1, n2) in edges {
        graph.update_edge(*nodes.get(&n1).unwrap(), *nodes.get(&n2).unwrap(), 1);
    }
    graph
}

fn remove_node(graph: &mut Graph<N, E, Undirected>, index: NodeIndex) {
    let neighbours = graph.neighbors(index).collect::<Vec<NodeIndex>>();
    for n1 in neighbours.clone() {
        for n2 in neighbours.clone() {
            if n1 >= n2 {
                continue;
            }
            let ex1 = graph.find_edge(index, n1).unwrap();
            let ex2 = graph.find_edge(index, n2).unwrap();
            let ew1 = *graph.edge_weight(ex1).unwrap();
            let ew2 = *graph.edge_weight(ex2).unwrap();
            graph.add_edge(n1, n2, ew1 + ew2);
        }
    }
    graph.remove_node(index);
}
fn reduce_graph(graph: &mut Graph<N, E, Undirected>) {
    let search = graph.node_indices().find(
        |ix| { let node = graph.node_weight(*ix).unwrap(); node.flow == 0 && !node.starting }
    );
    if let Some(index) = search {
        remove_node(graph, index);
        reduce_graph(graph);
    }
}

fn shortest_distances(graph: &Graph<N, E, Undirected>) -> HashMap<(NodeIndex, NodeIndex), usize> {
    let mut ret = HashMap::new();
    for ix in graph.node_indices() {
        for (ix2, length) in dijkstra(graph, ix, None, |e| *e.weight()) {
            ret.insert((ix, ix2), length);
        }
    }
    ret
}

fn start(graph: &Graph<N, E, Undirected>) -> NodeIndex {
    graph.node_indices().find(
        |ix| graph.node_weight(*ix).unwrap().starting
    ).unwrap()
}

fn upper_bound(config: &GraphDetails) -> usize {
    let mut ret = 0;
    for ix in config.remaining.clone() {
        let distance = *config.distances.get(&(config.position, ix)).unwrap();
        if distance + 1 > config.time {
            continue;
        }
        let pressure = config.graph.node_weight(ix).unwrap().flow;
        ret += pressure * (config.time - (distance + 1));
    }
    ret
}

fn process_routes(config: GraphDetails, lower_bound: &mut usize) {
    if config.pressure > *lower_bound {
        *lower_bound = config.pressure;
    }
    let ub = upper_bound(&config);
    if config.pressure + ub < *lower_bound {
        return;
    }
    for next in &config.remaining {
        let switch_time = config.distances.get(&(config.position, *next)).unwrap() + 1;
        if switch_time > config.time {
            continue;
        }
        let node_pressure = config.graph.node_weight(*next).unwrap().flow;
        let next_time = config.time - switch_time;
        let next_pressure = config.pressure + next_time * node_pressure;
        let next_remaining = config.remaining.iter().cloned().filter(|ix| *ix != *next).collect();
        process_routes(GraphDetails{
            graph: config.graph,
            distances: config.distances,
            position: *next,
            remaining: next_remaining,
            time: next_time,
            pressure: next_pressure,
        }, lower_bound);
    }
}

fn max_pressure_solo(graph: &Graph<N, E, Undirected>, time: usize) -> usize {
    let mut lower_bound = 0;
    let distances = &shortest_distances(graph);
    let position = start(graph);
    let remaining = graph.node_indices().filter(|ix| *ix != position).collect();
    process_routes(GraphDetails {
        graph, 
        distances,
        position,
        remaining,
        time,
        pressure: 0,
    }, &mut lower_bound);
    lower_bound
}

// ~30k sets in 15-node powerset is the current limiting factor in performance
fn max_pressure_with_elephant(graph: &Graph<N, E, Undirected>, time: usize) -> usize {
    let mut lower_bound = 0;
    let distances = &shortest_distances(graph);
    let position = start(graph);
    let remaining: Vec<NodeIndex> = graph.node_indices().filter(|ix| *ix != position).collect();
    for set in remaining.iter().map(|ix| ix.index()).powerset() {
        let my_remaining: Vec<NodeIndex> = set.iter().map(|ix| (*ix as u32).into()).collect();
        let elephant_remaining: Vec<NodeIndex> = remaining.iter().cloned().filter(|ix| !my_remaining.contains(ix)).collect();
        let mut my_lower_bound = 0;
        let mut elephant_lower_bound = 0;
        process_routes(GraphDetails {
            graph,
            distances,
            position,
            remaining: my_remaining,
            time,
            pressure: 0
        }, &mut my_lower_bound);
        process_routes(GraphDetails {
            graph,
            distances,
            position,
            remaining: elephant_remaining,
            time,
            pressure: 0
        }, &mut elephant_lower_bound);
        lower_bound = max(lower_bound, my_lower_bound + elephant_lower_bound);
    }
    lower_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parses_to_graph() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB\nValve BB has flow rate=0; tunnels lead to valves AA\n";
        let graph = parse_input(input);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn removes_node() {
        let input = "Valve AA has flow rate=10; tunnels lead to valves BB\nValve BB has flow rate=0; tunnels lead to valves AA, CC\nValve CC has flow rate=10; tunnels lead to valves BB\n";
        let mut graph = parse_input(input);
        remove_node(&mut graph, 1_u32.into());
        assert_eq!(graph.node_count(), 2);
        dbg!(&graph);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(*graph.edge_weight(0.into()).unwrap(), 2);
    }


    #[test]
    fn removes_unpressured_valves() {
        let input = "Valve AA has flow rate=10; tunnels lead to valves BB\nValve BB has flow rate=0; tunnels lead to valves AA, CC\nValve CC has flow rate=10; tunnels lead to valves BB\n";
        let mut graph = parse_input(input);
        reduce_graph(&mut graph);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(*graph.edge_weight(0.into()).unwrap(), 2);
    }

    #[test]
    fn finds_shortest_paths_between_pressured_valves() {
        let input = "Valve AA has flow rate=10; tunnels lead to valves BB\nValve BB has flow rate=10; tunnels lead to valves AA, CC\nValve CC has flow rate=10; tunnels lead to valves BB\n";
        let mut graph = parse_input(input);
        reduce_graph(&mut graph);
        let distances = shortest_distances(&graph);
        let expected: HashMap<(NodeIndex, NodeIndex), usize> = [
            ((0.into(), 0.into()), 0),
            ((0.into(), 1.into()), 1),
            ((0.into(), 2.into()), 2),
            ((1.into(), 0.into()), 1),
            ((1.into(), 1.into()), 0),
            ((1.into(), 2.into()), 1),
            ((2.into(), 0.into()), 2),
            ((2.into(), 1.into()), 1),
            ((2.into(), 2.into()), 0),
        ].iter().cloned().collect();
        assert_eq!(distances, expected);
    }

    #[test]
    fn process_minimal_route() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves BB\n";
        let mut graph = parse_input(input);
        reduce_graph(&mut graph);
        let time = 30;
        let mut lower_bound = 0;
        let distances = &shortest_distances(&graph);
        let position = start(&graph);
        let remaining = graph.node_indices().filter(|ix| *ix != position).collect();
        process_routes(GraphDetails {
            graph: &graph,
            distances,
            position,
            remaining,
            time,
            pressure: 0,
        }, &mut lower_bound);
        assert_eq!(lower_bound, 416);
    }
    #[test]
    fn calculate_example_route() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II\n";
        let mut graph = parse_input(input);
        reduce_graph(&mut graph);
        let time = 30;
        let mut lower_bound = 0;
        let distances = &shortest_distances(&graph);
        let position = start(&graph);
        let remaining = graph.node_indices().filter(|ix| *ix != position).collect();
        process_routes(GraphDetails {
            graph: &graph,
            distances,
            position,
            remaining,
            time,
            pressure: 0,
        }, &mut lower_bound);
        assert_eq!(lower_bound, 1651);
    }

    #[test]
    fn basic_upper_bound() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves BB\n";
        let graph = &parse_input(input);
        // reduce_graph(&mut graph);
        let position = 0.into();
        let distances: &HashMap<(NodeIndex, NodeIndex), usize> = &[
            ((0.into(), 0.into()), 0),
            ((0.into(), 1.into()), 10),
            ((0.into(), 2.into()), 20),
            ((1.into(), 0.into()), 10),
            ((1.into(), 1.into()), 0),
            ((1.into(), 2.into()), 10),
            ((2.into(), 0.into()), 20),
            ((2.into(), 1.into()), 10),
            ((2.into(), 2.into()), 0),
        ].iter().cloned().collect();
        let remaining = vec![1.into(), 2.into()];
        // (30-(10+1))*13 + (30-(20+1))*2
        let ub = upper_bound(&GraphDetails {
            graph,
            distances,
            position,
            remaining,
            time: 30,
            pressure: 0
        }
        );
        assert_eq!(ub, 265);
    }
}