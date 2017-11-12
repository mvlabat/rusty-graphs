use super::{GraphNode, GraphEdge};

use std::clone::Clone;
use std::cell::Ref;
use std::hash::Hash;
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

pub struct DijkstraPathInfo<Node> {
    pub node: Node,
    pub previous_node: Option<Node>,
    pub cost: i32,
}

impl<Node> DijkstraPathInfo<Node> {
    pub fn new(node: Node) -> DijkstraPathInfo<Node> {
        DijkstraPathInfo {
            node,
            previous_node: None,
            cost: i32::max_value(),
        }
    }
}

pub type DijkstraPathsResult<Node> = HashMap<Node, DijkstraPathInfo<Node>>;

pub fn dijkstra_paths<Datum, EdgeWeight, Node, Edge, F>(start_node: Node, convert_weight: F) -> DijkstraPathsResult<Node> where
    Node: GraphNode<Datum, EdgeWeight, Edge> + Clone + Hash,
    Edge: GraphEdge<Datum, EdgeWeight, Node>,
    F: Fn(Ref<EdgeWeight>) -> i32,
{
    let mut result = DijkstraPathsResult::<Node>::new();
    let mut visited_nodes: HashSet<Node> = HashSet::new();
    let mut unvisited_nodes: HashSet<Node> = HashSet::new();

    unvisited_nodes.insert(start_node.clone());
    result.insert(start_node.clone(), DijkstraPathInfo {
        node: start_node.clone(),
        previous_node: None,
        cost: 0,
    });

    while !unvisited_nodes.is_empty() {
        // Choosing which node to visit.
        let visited_node: Node = result.values()
            .filter(|path| !visited_nodes.contains(&path.node))
            .min_by(|path_x, path_y| path_x.cost.cmp(&path_y.cost))
            .unwrap()
            .node.clone();
        let path_cost = result.get(&visited_node).unwrap().cost;

        for edge in visited_node.edges().iter() {
            let neighbour = edge.opposite(&visited_node).clone();

            // Registering unknown nodes.
            if !visited_nodes.contains(&neighbour) && !unvisited_nodes.contains(&neighbour) {
                unvisited_nodes.insert(neighbour.clone());
            }
            let mut neighbour_path_info = result
                .entry(neighbour.clone())
                .or_insert(DijkstraPathInfo::new(neighbour.clone()));

            // Comparing to saved results and saving shorter paths.

            let new_path_cost = path_cost + convert_weight(edge.weight());
            if new_path_cost < neighbour_path_info.cost {
                neighbour_path_info.cost = new_path_cost;
                neighbour_path_info.previous_node = Some(visited_node.clone());
            }
        }

        unvisited_nodes.remove(&visited_node);
        visited_nodes.insert(visited_node);
    }

    result
}
