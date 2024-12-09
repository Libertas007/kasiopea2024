use std::collections::{HashMap, HashSet, VecDeque};
use graph::prelude::*;

use std::{fs, slice};
use std::ops::Index;
use std::sync::Mutex;
use std::str;
use std::time::Instant;
use indexmap::IndexMap;

pub fn run() {
    let start = Instant::now();
    let content = fs::read_to_string("G.txt").unwrap();

    let content = content.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();

    let problems: u32 = content.get(0).unwrap().parse().unwrap();
    let mut line_ptr: usize = 1;

    let mut output: String = String::new();

    for _i in 0..problems {
        let constants: Vec<usize> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
        let n = constants[0];
        let max = constants[1];
        let raw_towns = *content.get(line_ptr + 1).unwrap();
        let towns: Vec<&str> = raw_towns.split("").collect();
        line_ptr += 2;

        if raw_towns.replace("K", "").len() == 0 {
            output += &*n.to_string();
            output += "\n";
            continue;
        }

        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n);

        for i in 0..n - 1 {
            let values: Vec<usize> = content.get(line_ptr).unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
            edges.push((values[0], values[1]));
            line_ptr += 1;
        }

        let graph: UndirectedCsrGraph<usize, &str> = GraphBuilder::new().csr_layout(CsrLayout::Unsorted).edges(edges).node_values(towns.clone()).build();

        let mut safe_towns: Vec<usize> = vec![];
        for i in 0..towns.len() {
            if towns[i] == "K" {
                safe_towns.push(i);
            }
        }



        let mut prev_size = 0;
        let mut used_nodes = Vec::new();

        for a in &safe_towns {
            if used_nodes.contains(a) {
                continue;
            }

            let (val, size, nodes) = visit(&graph, &safe_towns, *a, 0, max);

            if size > prev_size {
                prev_size = size;
            }

            for b in nodes {
                used_nodes.push(b);
            }
        }


        println!("|");
        println!("{:?}", start.elapsed());

        output += &*prev_size.to_string();
        output += "\n";
    }

    println!("{:?}", start.elapsed());
    fs::write("G-out.txt", output.trim()).unwrap();
}

fn visit(graph: &UndirectedCsrGraph<usize, &str>, safe_towns: &Vec<usize>, node: usize, parent: usize, max: usize) -> (i32, i32, Vec<usize>) {
    let mut used = 0;
    let mut size = 1;
    let mut nodes = vec![node];

    for a in graph.neighbors(node) {
        if a == &parent {
            continue;
        }

        if safe_towns.contains(a) {
            let (val, new_size, new_nodes) = visit(&graph, safe_towns, *a, node, max);
            if val == -1 {
                return (-1, size, nodes);
            }

            used += val;
            size += new_size;
            for a in new_nodes {
                nodes.push(a);
            }
        } else {
            used += 1;
        }

        if used > max as i32 {
            return (1, 0, nodes);
        }
    }


    (used, size, nodes)
}