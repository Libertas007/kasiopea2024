use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use std::cmp::Ordering;
use std::{fs, slice};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::str;
use std::time::Instant;
use indexmap::IndexMap;
use pathfinding::prelude::{build_path, dijkstra_all};
use petgraph::{Graph, Undirected};
use petgraph::algo::{astar, dijkstra, Measure};
use petgraph::data::DataMap;
use petgraph::prelude::{NodeIndex, UnGraph};
use petgraph::visit::{EdgeRef, GraphBase, IntoEdges, VisitMap, Visitable};

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("E.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let n: usize = content.get(line_ptr).unwrap().parse().unwrap();
        let mut teams: Vec<usize> = content.get(line_ptr + 1).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        line_ptr += 2;

        teams.insert(0, 0);
        teams.push(0);


        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n as usize);

        let mut team_nodes: HashMap<usize, Vec<usize>> = HashMap::new();

        for i  in 0..teams.len() {
            let t = teams[i];

            if team_nodes.contains_key(&t) {
                team_nodes.get_mut(&t).unwrap().push(i);
            } else {
                team_nodes.insert(t, vec![i]);
            }
        }

        for i in 0..n - 1 {
            let values: Vec<usize> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
            edges.push((values[0], values[1]));
            line_ptr += 1;
        }

        let len = (teams.iter().max().unwrap() + 1);

        let graph: Graph<usize, usize, Undirected, usize> = UnGraph::from_edges(edges.iter());

        let mut player_distances: HashMap<usize, usize> = HashMap::new();

        /*let mut distances: HashMap<i32, Vec<i32>> = HashMap::new();

        distances.insert(n, vec![0; len]);*/

        let mut q = 0;

        /*let res: HashMap<usize, (usize, usize)> = dijkstra_all(&n, |e| graph.neighbors(NodeIndex::new(*e)).map(|f| (f.index(), 1)).collect::<Vec<_>>());
        //println!("{:?}", res);

        for (a, b) in res.clone() {
            build_path(&a, &res);
        }
*/

        //let distances = dijkstra(&graph, NodeIndex::new(n), None, |_| 1);

        let mut stack: VecDeque<usize> = VecDeque::new();

        for a in 1..graph.node_count() + 1 {
            let neighbours = graph.neighbors(NodeIndex::new(a)).count();
            if (neighbours == 1 || neighbours > 2) && a != n {
                stack.push_front(a);
            }
        }

        println!("{}", stack.len());

        while !stack.is_empty() {
            let me = stack.pop_back().unwrap();
            let path = astar(&graph, NodeIndex::new(me), |e| e.index() == n, |_| 1, |_| 0).unwrap();
            //println!("{:?}", path);

            q += 1;

            if q % 10000 == 0 {
                println!("{} {} {} {} {:?}", me, q, stack.len(), n, start.elapsed())
            }
        }


/*        let mut paths: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        let mut stack: VecDeque<(usize, Vec<(usize, usize)>)> = VecDeque::new();

        for a in 1..graph.node_count() + 1 {
            let neighbours = graph.neighbors(NodeIndex::new(a)).count();
            if (neighbours == 1 || neighbours > 2) && a != n {
                stack.push_front((a, vec![]));
            }
        }

        println!("{}", stack.len());

        while !stack.is_empty() {
            let (me, ancestors) = stack.pop_front().unwrap();

            let my_index = NodeIndex::new(me);
            let mut ancestors = ancestors.clone();

            ancestors.push((me, teams[me]));

            if me == n || (graph.neighbors(my_index).count() > 2 && ancestors.len() != 1) {
                paths.insert(ancestors.first().unwrap().0, ancestors);
                continue;
            }

            let my_distance = distances.get(&my_index).unwrap_or_else(|| panic!("{}, {:?}", me, ancestors));


            for a in graph.neighbors(my_index) {
                if my_distance < distances.get(&a).unwrap() {
                    continue;
                }

                stack.push_front((a.index(), ancestors.clone()));
            }

            q += 1;

            if q % 10000 == 0 {
                println!("{} {} {} {} {:?}", me, q, stack.len(), n, start.elapsed())
            }
        }*/



        //println!("{:?}", paths);

        //modified_dijkstra(&graph, NodeIndex::new(n), None);

        /*while !stack.is_empty() {
            let current = stack.pop_front().unwrap();
            let mut ancestors = current.2.clone();
            let mut ancestors_reverse = current.3.clone();
            let (&last_distance, &parent) = ancestors.last().unwrap_or((&0, &0));
            ancestors.insert(last_distance + 1, current.0);
            ancestors_reverse.insert(current.0, last_distance + 1);
            let unique_a = ancestors.values().collect::<HashSet<_>>();


            for idx in graph.neighbors(NodeIndex::new(current.0 as usize)) {
                let a = idx.index();
                let mut ancestors = ancestors.clone();
                let mut ancestors_reverse = ancestors_reverse.clone();

                if a == parent as usize || a == current.1 as usize {
                    continue;
                }

                let team = graph.node_weight(idx).unwrap();

                if ancestors.len() > 1 {
                    let unique_b = team_nodes.get(team).unwrap().iter().collect::<HashSet<&usize>>();

                    let c = unique_a.intersection(&unique_b).collect::<Vec<_>>();

                    //println!("{:?} {:?} {:?} {:?}", unique_a, unique_b, c, ancestors);

                    if c.len() != 0 {
                        let previous = *c.first().unwrap();
                        let pos = ancestors_reverse.get(*previous).unwrap();

                        player_distances.insert(*a, last_distance - pos + 2);

                        ancestors.shift_remove(pos);
                        ancestors_reverse.shift_remove(*previous);
                    } else {
                        player_distances.insert(*a, last_distance + 1);
                    }
                } else {
                    player_distances.insert(*a, 1);
                }

                /*if distances.contains_key(a) {
                    distances.insert(*a, distances[&a].iter().map(|e| e + 1).collect());
                } else {
                    distances.insert(*a, vec![1; len]);
                }

                player_distances[*a as usize] = distances[&a][*team as usize];
                distances.get_mut(&a).unwrap()[*team as usize] = 0;*/

                stack.push_front((*a, current.0, ancestors, ancestors_reverse));
            }

           /* distances.remove(&current.1);*/

            q += 1;

            if q % 10000 == 0 {
                println!("{} {} {} {} {:?}", current.0, q, stack.len(), n, start.elapsed())
            }
        }*/

        /*let player_distance = Mutex::new(vec![0; (n + 1) as usize]);
        let team_distance = vec![0; (n + 1) as usize];

        visit(&graph, &n, &-1, 0, &team_distance, &player_distance);

        player_distance.lock().unwrap().remove(0);
        player_distance.lock().unwrap().pop();

        for a in player_distance.lock().unwrap().iter() {
            output += &*a.to_string();
            output += " ";
        }*/

        player_distances.remove(&0);
        player_distances.remove(&n);

        println!("|");
        println!("{:?}", start.elapsed());
        let mut key_values: Vec<(&usize, &usize)> = player_distances.iter().collect();
        key_values.sort();

        output += &*key_values.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join(" ");
        output += "\n";
    }

    println!("{:?}", start.elapsed());
    fs::write("E-out.txt", output.trim()).unwrap();
}

/*fn visit(graph: &UndirectedCsrGraph<i32, i32>, node: &i32, parent: &i32, distance: i32, team_distance: &Vec<i32>, player_distance: &Mutex<Vec<i32>>) {
    let current_team = graph.node_value(*node);

    let mut team_distance = team_distance.clone();
    let mut player_distance_mutex = player_distance.lock().unwrap();
    player_distance_mutex[*node as usize] = team_distance[*current_team as usize];
    team_distance[*current_team as usize] = 0;

    drop(player_distance_mutex);

    for a in 0..team_distance.len() {
        team_distance[a] += 1;
    }

    let distance = distance + 1;


    for a in graph.neighbors(*node) {
        if a == parent {
            continue;
        }

        visit(&graph, a, node, distance, &team_distance, &player_distance);
    }
}*/

pub fn modified_dijkstra(
    graph: &Graph<usize, usize, Undirected, usize>,
    start: NodeIndex<usize>,
    goal: Option<NodeIndex<usize>>,
    //mut edge_cost: F,
) -> HashMap<NodeIndex<usize>, Vec<usize>>
where

//F: FnMut(G::EdgeRef) -> K,
    //K: Measure + Copy,
{
    let mut visited = graph.visit_map();
    let mut scores = HashMap::new();
    //let mut predecessor = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    scores.insert(start, Vec::new());
    visit_next.push(MinScored(Vec::new(), start));
    while let Some(MinScored(node_score, node)) = visit_next.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        if goal.as_ref() == Some(&node) {
            break;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }
            let mut next_score = node_score.clone();
            next_score.push(*graph.node_weight(next).unwrap());
            match scores.entry(next) {
                Occupied(ent) => {
                        *ent.into_mut() = next_score.clone();
                        visit_next.push(MinScored(next_score.clone(), next));
                        //predecessor.insert(next.clone(), node.clone());
                }
                Vacant(ent) => {
                    ent.insert(next_score.clone());
                    visit_next.push(MinScored(next_score.clone(), next));
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
        } else if a < b {
            Ordering::Greater
        } else if a > b {
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
