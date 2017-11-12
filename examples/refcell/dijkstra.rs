extern crate graph;
extern crate refcell;

use std::string::String;
use std::cell::Ref;

use graph::GraphNode;
use graph::algo::{dijkstra_paths, DijkstraPathInfo};
use std::collections::btree_map::BTreeMap;

use refcell::GraphNode as RcGraphNode;

fn main() {
    let mut a: RcGraphNode<String, i32> = RcGraphNode::create(String::from("A"));
    let mut b = a.add(String::from("B"), 6);
    let mut c = b.add(String::from("C"), 5);
    let mut d = a.add(String::from("D"), 1);
    let e = d.add(String::from("E"), 1);
    b.connect(d.clone(), 2);
    b.connect(e.clone(), 2);
    c.connect(e.clone(), 5);

    let sorted_result = {
        let mut result = dijkstra_paths(a, |weight: Ref<i32>| -> i32 { *weight });

        let sorted_result: BTreeMap<String, DijkstraPathInfo<RcGraphNode<String, i32>>> = result
            .drain()
            .map(|(node, info)| (node.datum().clone(), info))
            .collect();

        sorted_result
    };

    for path_info in sorted_result.values() {
        let previous_node_string: String = match path_info.previous_node {
            Some(ref node) => node.datum().clone(),
            None => String::from("None"),
        };

        println!("[{}] {} - {}",
            path_info.node.datum(),
            path_info.cost,
            previous_node_string);
    }
}
