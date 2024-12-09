use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use std::cmp::Ordering;
use std::{fs, slice};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::ptr::eq;
use std::str;
use std::time::Instant;
use indexmap::IndexMap;
use pathfinding::prelude::{build_path, dijkstra_all};
use petgraph::{Graph, Undirected};
use petgraph::algo::{astar, dijkstra, Measure};
use petgraph::data::{Build, DataMap};
use petgraph::prelude::{NodeIndex, UnGraph};
use petgraph::visit::{EdgeRef, GraphBase, IntoEdges, VisitMap, Visitable};

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("I.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let values: Vec<usize> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 1;

        let n = *values.get(0).unwrap();
        let m = *values.get(1).unwrap();

        println!("{}, {}", n, m);

        if n == 2 && m == 1 {
            line_ptr += 1;
            output += &*"-1";
            output += "\n";
            continue;
        }

        let mut cold_edges: Vec<(usize, usize, usize)> = Vec::new();
        let mut warm_edges: Vec<(usize, usize, usize)> = Vec::new();

        for i in 0..m {
            let values: Vec<&str> = content.get(line_ptr).unwrap().split(" ").collect();

            let a = values[0].parse().unwrap();
            let b = values[1].parse().unwrap();
            let c = values[2].parse().unwrap();

            if values[3] == "C" {
                cold_edges.push((a, b, c));
            } else {
                warm_edges.push((a, b, c));
            }
            line_ptr += 1;
        }

        let mut cold_graph: Graph<usize, usize, Undirected, usize> = UnGraph::from_edges(cold_edges.iter());
        let mut warm_graph: Graph<usize, usize, Undirected, usize> = UnGraph::from_edges(warm_edges.iter());

        for i in 0..n {
            warm_graph.add_node(0);
        }

        for i in 0..n {
            cold_graph.add_node(0);
        }

        let cold_paths = modified_dijkstra(&cold_graph, NodeIndex::new(n), None, |e| *e.weight());
        let warm_paths = modified_dijkstra(&warm_graph, NodeIndex::new(n), None, |e| *e.weight());
        //println!("{:?}\n{:?}", cold_paths, warm_paths);

        let cold_paths = cold_paths.iter().filter(|&e| e.1 != &usize::MAX).map(|e| (e.0.index(), *e.1)).collect::<HashMap<_, _>>();
        let warm_paths = warm_paths.iter().filter(|&e| e.1 != &usize::MAX).map(|e| (e.0.index(), *e.1)).collect::<HashMap<_, _>>();

        if cold_paths.len() == 0 || warm_paths.len() == 0 {
            output += &*"-1";
            output += "\n";
            continue;
        }

        let mut paths = Vec::new();

        for i in 1..n {
            if warm_paths.contains_key(&i) && cold_paths.contains_key(&i) {
                paths.push(warm_paths.get(&i).unwrap() + cold_paths.get(&i).unwrap());
            }
        }

        println!("|");
        println!("{:?}", start.elapsed());

        let max = *paths.iter().max().unwrap_or(&0);

        if max == 0 {
            output += &*"-1";
            output += "\n";
            continue;
        }

        output += &*max.to_string();
        output += "\n";
    }

    println!("{:?}", start.elapsed());
    fs::write("I-out.txt", output.trim()).unwrap();
}

pub fn modified_dijkstra<G, F>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
    mut edge_cost: F,
) -> HashMap<G::NodeId, usize>
where
    G: IntoEdges + Visitable,
    G::NodeId: Eq + Hash,
    F: FnMut(G::EdgeRef) -> usize,
{
    let mut visited = graph.visit_map();
    let mut scores = HashMap::new();
    //let mut predecessor = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    let zero_score = usize::MAX;
    scores.insert(start, zero_score);
    visit_next.push(MinScored(zero_score, start));
    while let Some(MinScored(node_score, node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        if goal.as_ref() == Some(&node) {
            break;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            /*if visited.is_visited(&next) {
                continue;
            }*/
            let next_score = node_score.min(edge_cost(edge));
            match scores.entry(next) {
                Occupied(ent) => {
                    if next_score > *ent.get() {
                        *ent.into_mut() = next_score;
                        visit_next.push(MinScored(next_score, next));
                        //predecessor.insert(next.clone(), node.clone());
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    visit_next.push(MinScored(next_score, next));
                    //predecessor.insert(next.clone(), node.clone());
                }
            }
        }
        visited.visit(node);
    }
    scores
}

/// `MinScored<K, T>` holds a score `K` and a scored object `T` in
/// a pair for use with a `BinaryHeap`.
///
/// `MinScored` compares in reverse order by the score, so that we can
/// use `BinaryHeap` as a min-heap to extract the score-value pair with the
/// least score.
///
/// **Note:** `MinScored` implements a total order (`Ord`), so that it is
/// possible to use float types as scores.
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a > b {
            Ordering::Greater
        } else if a < b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
