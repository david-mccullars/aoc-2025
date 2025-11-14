use num::Bounded;
use num::Num;
use num::Saturating;
use num::Zero;
use petgraph::EdgeType;
use petgraph::Graph;
use petgraph::dot::Dot;
use petgraph::graph::IndexType;
use petgraph::graph::NodeIndex;
use petgraph::visit::GraphProp;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeIndexable;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn write_graph<G>(graph: G, filename: &str)
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: std::fmt::Display,
    G::NodeWeight: std::fmt::Display,
{
    // Convert the graph into DOT format
    // let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let dot = Dot::with_config(graph, &[]);
    let dot_string = format!("{}", dot);

    // Write the DOT string to a temporary file
    let dot_filename = "temp.dot";
    let mut file = File::create(dot_filename).expect("Unable to create file");
    file.write_all(dot_string.as_bytes())
        .expect("Unable to write data");

    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    // Use the `dot` command-line tool (part of the Graphviz software package) to convert the DOT file to SVG
    let output = Command::new("dot")
        .arg(format!("-T{}", ext).as_str())
        .arg(dot_filename)
        .arg("-o")
        .arg(filename)
        .output()
        .expect("Failed to execute command");

    // Check the output of the command
    if !output.status.success() {
        eprintln!(
            "dot command failed with output:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Delete the temporary DOT file
    std::fs::remove_file(dot_filename).expect("Failed to remove temporary file");
}

pub fn floyd_warshall<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
) -> HashMap<(NodeIndex<Ix>, NodeIndex<Ix>), E>
where
    E: Num + Bounded + Zero + Saturating + PartialOrd + Copy,
    Ty: EdgeType,
    Ix: IndexType,
{
    let mut distances = HashMap::new();
    let nodes: Vec<_> = graph.node_identifiers().collect();

    // Initialize distances
    for node in &nodes {
        for target in &nodes {
            if node == target {
                distances.insert((*node, *target), E::zero());
            } else if let Some(edge) = graph.find_edge(*node, *target) {
                distances.insert((*node, *target), graph[edge]);
            } else {
                distances.insert((*node, *target), E::max_value());
            }
        }
    }

    // Floyd-Warshall algorithm
    for k in &nodes {
        for i in &nodes {
            for j in &nodes {
                let ikj = distances[&(*i, *k)].saturating_add(distances[&(*k, *j)]);
                if ikj < distances[&(*i, *j)] {
                    distances.insert((*i, *j), ikj);
                }
            }
        }
    }

    distances
}

pub fn shortest_hamiltonian_path<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
    start: NodeIndex<Ix>,
    require_finish_at_start: bool,
) -> Option<(Vec<NodeIndex<Ix>>, E)>
where
    E: Num + Bounded + Zero + Saturating + PartialOrd + Copy,
    Ty: EdgeType,
    Ix: IndexType,
{
    let nodes: Vec<_> = graph.node_identifiers().collect();
    let n = nodes.len();

    if n == 0 {
        return None;
    }

    if n == 1 {
        if nodes[0] == start {
            return Some((vec![start], E::zero()));
        } else {
            return None;
        }
    }

    let mut node_to_idx = HashMap::new();
    for (i, &node) in nodes.iter().enumerate() {
        node_to_idx.insert(node, i);
    }

    let start_idx = *node_to_idx.get(&start)?;

    let mut dist = vec![vec![E::max_value(); n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dist[i][j] = E::zero();
            } else if let Some(edge) = graph.find_edge(nodes[i], nodes[j]) {
                dist[i][j] = graph[edge];
            }
        }
    }

    let mut dp: HashMap<(usize, usize), E> = HashMap::new();
    let mut parent: HashMap<(usize, usize), usize> = HashMap::new();

    dp.insert((1 << start_idx, start_idx), E::zero());

    for mask in 1..(1 << n) {
        if mask & (1 << start_idx) == 0 {
            continue;
        }

        let bits: Vec<_> = (0..n).filter(|i| mask & (1 << i) != 0).collect();

        for &v in &bits {
            if v == start_idx && mask == (1 << start_idx) {
                continue;
            }

            let mask_without_v = mask ^ (1 << v);
            if mask_without_v == 0 {
                continue;
            }

            let mut min_cost = E::max_value();
            let mut best_u = None;

            for &u in &bits {
                if u == v {
                    continue;
                }

                let prev_cost = dp
                    .get(&(mask_without_v, u))
                    .copied()
                    .unwrap_or(E::max_value());
                if prev_cost == E::max_value() || dist[u][v] == E::max_value() {
                    continue;
                }

                let cost = prev_cost.saturating_add(dist[u][v]);
                if cost < min_cost {
                    min_cost = cost;
                    best_u = Some(u);
                }
            }

            if min_cost < E::max_value() {
                dp.insert((mask, v), min_cost);
                if let Some(u) = best_u {
                    parent.insert((mask, v), u);
                }
            }
        }
    }

    let full_mask = (1 << n) - 1;
    let mut min_cost = E::max_value();
    let mut last_node = None;

    for v in 0..n {
        if let Some(&cost) = dp.get(&(full_mask, v)) {
            let total_cost = if require_finish_at_start && v != start_idx {
                // Add cost to return to start
                if dist[v][start_idx] == E::max_value() {
                    continue; // No edge back to start
                }
                cost.saturating_add(dist[v][start_idx])
            } else {
                cost
            };

            if total_cost < min_cost {
                min_cost = total_cost;
                last_node = Some(v);
            }
        }
    }

    if min_cost == E::max_value() {
        return None;
    }

    let mut path = Vec::new();
    let mut current = last_node.unwrap();
    let mut mask = full_mask;

    while mask != 0 {
        path.push(nodes[current]);

        if mask == (1 << current) {
            break;
        }

        if let Some(&prev) = parent.get(&(mask, current)) {
            mask ^= 1 << current;
            current = prev;
        } else {
            break;
        }
    }

    path.reverse();

    // Add start node at the end if we're doing TSP
    if require_finish_at_start && !path.is_empty() && path.last() != Some(&start) {
        path.push(start);
    }

    Some((path, min_cost))
}
