extern crate graph;
extern crate refcell;

use std::string::String;

use graph::{GraphNode, GraphEdge};
use refcell::GraphNode as RcGraphNode;

fn main() {
    let mut root: RcGraphNode<String, i32> = RcGraphNode::create(String::from("root"));
    root.add(String::from("test1"), 2);
    root.add(String::from("test2"), 3);

    for edge in root.edges().iter() {
        let (node1, node2) = edge.nodes();
        println!("datum-1: {}; datum-2: {}; price: {}",
            node1.datum(), node2.datum(), edge.weight());
    }
}
