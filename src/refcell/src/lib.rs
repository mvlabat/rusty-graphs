extern crate graph;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

//#[derive(Clone)]
struct GraphNode<Datum, EdgeCost>(Rc<RefCell<Node<Datum, EdgeCost>>>);

impl<Datum, EdgeCost> Clone for GraphNode<Datum, EdgeCost> {
    fn clone(&self) -> GraphNode<Datum, EdgeCost> {
        GraphNode(self.0.clone())
    }
}

impl<Datum, EdgeCost> graph::GraphNode<Datum, EdgeCost, GraphEdge<Datum, EdgeCost>> for GraphNode<Datum, EdgeCost>
    where GraphEdge<Datum, EdgeCost>: graph::GraphEdge<Datum, GraphNode<Datum, EdgeCost>, EdgeCost>
{
    fn create(datum: Datum) -> GraphNode<Datum, EdgeCost> {
        GraphNode(Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        })))
    }

    fn add(&mut self, datum: Datum, cost: EdgeCost) -> GraphNode<Datum, EdgeCost> {
        let mut node = Self::create(datum);
        let mut edge = GraphEdge(Rc::new(RefCell::new(Edge {
            nodes: (self.clone(), node.clone()),
            cost,
        })));
        node.0.borrow_mut().edges.push(edge.clone());
        //        let edge = Edge { nodes: vec![Rc::new(self]}
        self.0.borrow_mut().edges.push(edge);
        node
    }

    fn datum(&self) -> &Datum {
        unsafe { &self.0.into_inner().datum }
    }

    fn edges(&self) -> &Vec<GraphEdge<Datum, EdgeCost>> {
        unsafe { &self.0.into_inner().edges }
    }

    fn print_edges(&self) {
//        let edges_string = self.edges.iter()
//            .map(|edge| { (*edge.borrow()).datum.to_string() })
//            .collect::<Vec<String>>()
//            .join("-");
//        println!("{}", edges_string);
    }
}

struct GraphEdge<Datum, EdgeCost>(RcEdge<Datum, EdgeCost>);

impl<Cost, Datum> graph::GraphEdge<Datum, GraphNode<Datum, Cost>, Cost> for GraphEdge<Datum, Cost>
    where GraphNode<Datum, Cost>: graph::GraphNode<Datum, Cost, GraphEdge<Datum, Cost>>
{
    fn nodes(&self) -> (GraphNode<Datum, Cost>, GraphNode<Datum, Cost>) {
        self.0.borrow().nodes
    }

    fn cost(&self) -> Cost {
        self.0.borrow().cost
    }
}

impl<Cost, Datum> Clone for GraphEdge<Cost, Datum> {
    fn clone(&self) -> GraphEdge<Cost, Datum> {
        *self
    }
}

//type RcGraphNode<T> = GraphNode<T, >
type WeakNode<Datum, EdgeCost> = Weak<RefCell<Node<Datum, EdgeCost>>>;
type RcEdge<Datum, EdgeCost> = Rc<RefCell<Edge<Datum, EdgeCost>>>;

pub struct Node<Datum, EdgeCost> {
    datum: Datum,
    edges: Vec<GraphEdge<Datum, EdgeCost>>,
}

struct Edge<Datum, EdgeCost> {
    nodes: (GraphNode<Datum, EdgeCost>, GraphNode<Datum, EdgeCost>),
    cost: EdgeCost,
}

//pub trait Node {
//    fn create(datum: &'static str) -> Node;
//    fn add(&self,)
//}

//impl<T, U> graph::GraphNode<T, U> for GraphNode<T> {
//    fn create(datum: T) -> GraphNode<T> {
//        GraphNode(Rc::new(RefCell::new(Node {
//            datum,
//            edges: Vec::new(),
//        })))
//    }
//
//    fn add(&mut self, datum: T) -> GraphNode<T> {
//        let mut node = Self::create(datum);
////        let edge = Edge { nodes: vec![Rc::new(self]}
//        self.0.edges.push(node.clone());
//        node
//    }
//
//    fn edges(&self) -> Vec<U> {
//
//    }
//
//    fn print_edges(&self) {
//        let edges_string = self.edges.iter()
//            .map(|edge| { (*edge.borrow()).datum.to_string() })
//            .collect::<Vec<String>>()
//            .join("-");
//        println!("{}", edges_string);
//    }
//}
