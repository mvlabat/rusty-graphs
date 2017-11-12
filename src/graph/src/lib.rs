mod dijkstra;

use std::cell::Ref;
use std::cmp::Eq;

pub mod algo {
    pub use dijkstra::{dijkstra_paths, DijkstraPathInfo};
}

pub trait GraphNode<Datum, EdgeWeight, Edge> where
    Self: std::marker::Sized + Eq,
    Edge: GraphEdge<Datum, EdgeWeight, Self>
{
    fn create(datum: Datum) -> Self;
    fn add(&mut self, datum: Datum, weight: EdgeWeight) -> Self;
    fn connect(&mut self, node: Self, weight: EdgeWeight);
    fn datum(&self) -> Ref<Datum>;
    fn edges(&self) -> Ref<Vec<Edge>>;
}

pub trait GraphEdge<Datum, EdgeWeight, Node> where
    Self: std::marker::Sized,
    Node: GraphNode<Datum, EdgeWeight, Self>
{
    fn nodes(&self) -> (Node, Node);
    fn weight(&self) -> Ref<EdgeWeight>;
    fn opposite(&self, node: &Node) -> Node {
        let (first_node, second_node) = self.nodes();
        if first_node == *node { second_node } else { first_node }
    }
}
