extern crate refcell;

//use std::cell::RefMut;

fn main() {
    let mut root = refcell::GraphNode::create("root");
    root.add("test1");
    root.add("test2");
    root.print_edges();
}
