use plotters::prelude::*;
use std::collections::HashMap;

pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
    }

    pub fn compute_degrees(&self) -> HashMap<usize, usize> {
        let mut degrees = HashMap::new();
        for (node, neighbors) in &self.adjacency_list {
            degrees.insert(*node, neighbors.len());
        }
        degrees
    }
}



    Ok(())
}

