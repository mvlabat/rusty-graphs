extern crate refcell;

fn main() {
    let root = refcell::Node::create("root");
    let mut root_node = &mut *root.borrow_mut();
    root_node.add("test1");
    root_node.add("test2");
    root_node.print_edges();
}
