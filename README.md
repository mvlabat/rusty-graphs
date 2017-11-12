# rusty-graphs
This library is created for learning purposes only and it's not supposed to be used in production
(well, you can of course, but I doubt if you should).

Rusty-graphs provides `graph` crate that has `GraphNode` and `GraphEdge` traits defined
and it also implements Dijkstra's path search algorithm. The mentioned traits allows you to define your own
graph implementation.

This library also provides you `refcell` crate, that implements a graph working on `Rc<RefCell<_>>` nodes.
This article was the main source of inspiration: https://github.com/nrc/r4cppp/tree/master/graphs

See the source code and the examples for implementation and usage details.

## Running examples
### Refcell
There are the following examples avaiable:
- graph
- dijkstra

```bash
cargo run -p refcell-examples --bin <example>
```
