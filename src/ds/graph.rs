#![allow(unused)]

// directed graph representation using adjacency list

use std::{collections::{HashMap, HashSet}, hash::Hash};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vertex<T: Hash + Eq + Clone> {
    value: T,
}

impl<T: Hash + Eq + Clone> Vertex<T> {
    fn new(value: T) -> Self {
        Vertex { value }
    }
}

struct DirectedGraph<T: Hash + Eq + Clone> {
    adjacency_table: HashMap<Vertex<T>, Vec<Vertex<T>>>
}

impl<T: Hash + Eq + Clone> DirectedGraph<T> {
    fn new() -> Self {
        DirectedGraph { adjacency_table: HashMap::new() }
    }

    fn add_node(&mut self, vertex: Vertex<T>) {
        match self.adjacency_table.get(&vertex) {
            None => {
                self.adjacency_table.insert(vertex, vec![]);
            },
            Some(_) => return,
        }
    }

    fn add_edge(&mut self, edge: (Vertex<T>, Vertex<T>)) {
        // if they already exist, nothing happens, otherwise they get inserted
        self.add_node(edge.0.clone());
        self.add_node(edge.1.clone());

        self.adjacency_table
        .entry(edge.0)
        .and_modify(|val| val.push(edge.1));
    }

    fn vertexes(&self) -> HashSet<&Vertex<T>> {
        self.adjacency_table.keys().collect()
    }

    fn edges(&self) -> Vec<(&Vertex<T>, &Vertex<T>)> {
        let mut edges = Vec::new();

        for (node, neighbors) in self.adjacency_table.iter() {
            for neighbor in neighbors {
                edges.push((node, neighbor));
            }
        }

        edges
    }
}

#[cfg(test)]
mod test_graph_ds {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_edge((Vertex::new(5), Vertex::new(10)));

        let expected_nodes = vec![Vertex::new(5), Vertex::new(10)];

        for vertex in expected_nodes.iter() {
            assert!(graph.vertexes().contains(vertex));
        }
    }

    #[test]
    fn test_neighbors() {
        let mut graph = DirectedGraph::new();
        graph.add_edge((Vertex::new(5), Vertex::new(10)));
        graph.add_edge((Vertex::new(10), Vertex::new(12)));
        graph.add_edge((Vertex::new(12), Vertex::new(5)));

        let expected_edges = vec![
            (Vertex::new(5), Vertex::new(10)),
            (Vertex::new(10), Vertex::new(12)),
            (Vertex::new(12), Vertex::new(5))
        ];

        for edge in graph.edges().iter() {
            println!("{:?} -> {:?}", edge.0, edge.1);
        }
    }
}