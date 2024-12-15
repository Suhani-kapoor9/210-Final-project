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


pub fn plot_centrality_no_labels(
    centrality: &HashMap<usize, usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("centrality_visualization_no_labels.png", (1024, 768))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let max_centrality = centrality.values().cloned().max().unwrap_or(0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Node Centrality Visualization Without Labels", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..centrality.len(), 0..(max_centrality + 10))?;

    chart.configure_mesh().draw()?;

    
    chart.draw_series(
        centrality
            .iter()
            .map(|(&node, &score)| Circle::new((node, score), 5, BLUE.filled())),
    )?;

    Ok(())
}


pub fn plot_centrality(
    centrality: &HashMap<usize, usize>,
    labels: &HashMap<usize, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("centrality_visualization_us_labeled.png", (1024, 768))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let max_centrality = centrality.values().cloned().max().unwrap_or(0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Node Centrality Visualization with US Labels", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..labels.len(), 0..(max_centrality + 10))?;

    chart.configure_mesh().draw()?;

  
    for (&node, &score) in centrality {
        if let Some(label) = labels.get(&node) {
            if label == "United States" {
               
                chart.draw_series(std::iter::once(Circle::new((node, score), 5, BLUE.filled())))?;

         
                chart.draw_series(std::iter::once(
                    Text::new(
                        format!("US: {}", score),
                        (node, score + 5),
                        ("sans-serif", 12).into_font(),
                    ),
                ))?;
            }
        }
    }

    Ok(())
}



