use std::cell::Ref;

pub trait GraphNode<Datum, EdgeWeight, Edge>
    where Self: std::marker::Sized,
          Edge: GraphEdge<Datum, EdgeWeight, Self>
{
    fn create(datum: Datum) -> Self;
    fn add(&mut self, datum: Datum, weight: EdgeWeight) -> Self;
    fn datum(&self) -> Ref<Datum>;
    fn edges(&self) -> Ref<Vec<Edge>>;
}

pub trait GraphEdge<Datum, EdgeWeight, Node>
    where Self: std::marker::Sized,
          Node: GraphNode<Datum, EdgeWeight, Self>
{
    fn nodes(&self) -> (Node, Node);
    fn weight(&self) -> Ref<EdgeWeight>;
}
