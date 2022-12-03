use std::collections::{BinaryHeap, HashSet, HashMap};

fn main() {
    println!("Hello, world!");
}

type Node = usize;

type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node,Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {

        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes: HashSet<Node> = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());
            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph{ edges: adjacency_list, nodes }
    }
}

#[derive(PartialEq, Eq)]
struct Step {
    cost: Cost,
    position: Node,
    track: Vec<Node>,
}

impl Ord for Step {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(graph: &Graph, start: Node, destination: Node) -> Option<(Vec<Node>, Cost)> {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Step{ cost: 0, position: start, track: vec![]});

    let mut distances: HashMap<Node, Cost> = graph.nodes
        .iter()
        .map(|&node| {
        if node == start {
            (node, 0)
        } else {
            (node, usize::MAX)
        }
    })
    .collect();

    while let Some(Step { cost, position, mut track }) = priority_queue.pop() {
        if position == destination {
            track.push(destination);
            return Some((track, cost));
        }

        if let Some(destinations) = graph.edges.get(&position) {
            for  &(next_node, next_cost) in destinations.iter() {
                if next_cost < distances[&next_node] {
                    let mut next_step = Step{
                        cost: cost + next_cost,
                        position: next_node,
                        track: track.clone(),
                    };

                    next_step.track.push(position);
                    priority_queue.push(next_step);
                    if let Some(old_cost) = distances.get_mut(&next_node) {
                        *old_cost = next_cost;
                    }
                }
            }
        }
    }

    None
}

#[test]
fn from_1000_to_9000() {
    let edge_list = include!("large_graph.in");
    let graph = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&graph, 1000, 9000);
    assert_eq!(path.clone().unwrap().1, 24);
    assert_eq!(path.unwrap().0.len(), 6);
}
