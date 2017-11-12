extern crate graph;

use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::hash::{Hash, Hasher};

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
        let node = Self::create(datum);
        self.connect(node.clone(), weight);
        node
    }

    fn connect(&mut self, node: Self, weight: EdgeWeight) {
        let edge = GraphEdge(Rc::new(RefCell::new(Edge {
            nodes: (self.clone(), node.clone()),
            weight,
        })));
        node.0.borrow_mut().edges.push(edge.clone());
        self.0.borrow_mut().edges.push(edge);
    }

    fn datum(&self) -> Ref<Datum> {
        Ref::map(self.0.borrow(), |node| &node.datum)
    }

    fn edges(&self) -> Ref<Vec<GraphEdge<Datum, EdgeWeight>>> {
        Ref::map(self.0.borrow(), |node| &node.edges)
    }
}

impl<Datum, EdgeWeight> PartialEq for GraphNode<Datum, EdgeWeight> {
    fn eq(&self, other: &GraphNode<Datum, EdgeWeight>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<Datum, EdgeWeight> Eq for GraphNode<Datum, EdgeWeight> {}

impl<Datum, EdgeWeight> Hash for GraphNode<Datum, EdgeWeight> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::into_raw(self.0.clone()).hash(state);
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

type RcEdge<Datum, EdgeWeight> = Rc<RefCell<Edge<Datum, EdgeWeight>>>;

pub struct Node<Datum, EdgeWeight> {
    datum: Datum,
    edges: Vec<GraphEdge<Datum, EdgeWeight>>,
}

struct Edge<Datum, EdgeWeight> {
    nodes: (GraphNode<Datum, EdgeWeight>, GraphNode<Datum, EdgeWeight>),
    weight: EdgeWeight,
}
