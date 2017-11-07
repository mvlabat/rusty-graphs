extern crate graph;

use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};

pub struct GraphNode<Datum, EdgeWeight>(Rc<RefCell<Node<Datum, EdgeWeight>>>);

impl<Datum, EdgeWeight> Clone for GraphNode<Datum, EdgeWeight> {
    fn clone(&self) -> GraphNode<Datum, EdgeWeight> {
        GraphNode(self.0.clone())
    }
}

impl<Datum, EdgeWeight> graph::GraphNode<Datum, EdgeWeight, GraphEdge<Datum, EdgeWeight>> for GraphNode<Datum, EdgeWeight>
{
    fn create(datum: Datum) -> GraphNode<Datum, EdgeWeight> {
        GraphNode(Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        })))
    }

    fn add(&mut self, datum: Datum, weight: EdgeWeight) -> GraphNode<Datum, EdgeWeight> {
        let mut node = Self::create(datum);
        let mut edge = GraphEdge(Rc::new(RefCell::new(Edge {
            nodes: (self.clone(), node.clone()),
            weight,
        })));
        node.0.borrow_mut().edges.push(edge.clone());
        self.0.borrow_mut().edges.push(edge);
        node
    }

    fn datum(&self) -> Ref<Datum> {
        Ref::map(self.0.borrow(), |node| &node.datum)
    }

    fn edges(&self) -> Ref<Vec<GraphEdge<Datum, EdgeWeight>>> {
        Ref::map(self.0.borrow(), |node| &node.edges)
    }
}

pub struct GraphEdge<Datum, EdgeWeight>(RcEdge<Datum, EdgeWeight>);

impl<Weight, Datum> graph::GraphEdge<Datum, Weight, GraphNode<Datum, Weight>> for GraphEdge<Datum, Weight>
{
    fn nodes(&self) -> (GraphNode<Datum, Weight>, GraphNode<Datum, Weight>) {
        let edge = self.0.borrow();
        (edge.nodes.0.clone(), edge.nodes.1.clone())
    }

    fn weight(&self) -> Ref<Weight> {
        Ref::map(self.0.borrow(), |edge| &edge.weight)
    }
}

impl<Weight, Datum> Clone for GraphEdge<Weight, Datum> {
    fn clone(&self) -> GraphEdge<Weight, Datum> {
        GraphEdge(self.0.clone())
    }
}

type WeakNode<Datum, EdgeWeight> = Weak<RefCell<Node<Datum, EdgeWeight>>>;
type RcEdge<Datum, EdgeWeight> = Rc<RefCell<Edge<Datum, EdgeWeight>>>;

pub struct Node<Datum, EdgeWeight> {
    datum: Datum,
    edges: Vec<GraphEdge<Datum, EdgeWeight>>,
}

struct Edge<Datum, EdgeWeight> {
    nodes: (GraphNode<Datum, EdgeWeight>, GraphNode<Datum, EdgeWeight>),
    weight: EdgeWeight,
}
