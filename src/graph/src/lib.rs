pub trait GraphNode<Datum, EdgeCost, Edge: GraphEdge<Datum, Self, EdgeCost>>
    where Self: std::marker::Sized
{
    fn create(datum: Datum) -> Self;
    fn add(&mut self, datum: Datum, cost: EdgeCost) -> Self;
    fn datum(&self) -> &Datum;
    fn edges(&self) -> &Vec<Edge>;
    fn print_edges(&self);
}

pub trait GraphEdge<Datum, T: GraphNode<Datum, Cost, Self>, Cost> where Self: std::marker::Sized {
    fn nodes(&self) -> (T, T);
    fn cost(&self) -> Cost;
}
