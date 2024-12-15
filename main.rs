use csv::Reader;
use std::collections::HashMap;
use std::error::Error;
use plotters::prelude::*;
mod graph;
use graph::{Graph, plot_centrality,plot_centrality_no_labels};



fn main() -> Result<(), Box<dyn Error>> {
    
    let mut reader = Reader::from_path("companies1.csv")?;

    let mut marketcap = Vec::new();
    let mut prices = Vec::new();
    let mut countries = Vec::new();

    
    for record in reader.records() {
        let record = record?;
        if let (Ok(mc), Ok(price), Ok(country)) = (
            record[3].parse::<f64>(),
            record[4].parse::<f64>(),
            record[5].parse::<String>(),
        ) {
            marketcap.push(mc);
            prices.push(price);
            countries.push(country);
        }
    }

   
    println!("Market Cap Distribution:");
    let max_cap = marketcap.iter().cloned().fold(f64::MIN, f64::max);
    let bins = 10;
    let bin_width = max_cap / bins as f64;
    let mut histogram = vec![0; bins];
    for &cap in &marketcap {
        let bin = (cap / bin_width).floor() as usize;
        if bin < bins {
            histogram[bin] += 1;
        }
    }
    for (i, &count) in histogram.iter().enumerate() {
        println!("Bin {}: {}", i, count);
    }

   
    let root = BitMapBackend::new("marketcap_distribution.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Market Cap Distribution", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..bins, 0..*histogram.iter().max().unwrap())?;
    chart.configure_mesh().draw()?;
    chart.draw_series(
        histogram.iter().enumerate().map(|(i, &count)| {
            Rectangle::new(
                [(i, 0), (i + 1, count)],
                ShapeStyle {
                    color: BLUE.mix(0.5).to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        }),
    )?;

    
    println!("\nGlobal Representation by Country:");
    let mut country_map: HashMap<String, usize> = HashMap::new();
    for country in &countries {
        *country_map.entry(country.clone()).or_default() += 1;
    }
    for (country, &count) in &country_map {
        println!("Country: {}, Companies: {}", country, count);
    }

  
    let root = BitMapBackend::new("global_representation.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Global Representation by Country", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..country_map.len(),
            0..*country_map.values().max().unwrap(),
        )?;
    chart.configure_mesh().draw()?;
    chart.draw_series(
        country_map.values().enumerate().map(|(i, &count)| {
            Rectangle::new(
                [(i, 0), (i + 1, count)],
                ShapeStyle {
                    color: GREEN.mix(0.5).to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        }),
    )?;

    
    println!("\nOutliers (High Market Cap or Stock Price):");
    let cap_threshold = max_cap * 0.9; // Top 10% market caps as outliers
    let mut outliers = Vec::new();
    for (i, &cap) in marketcap.iter().enumerate() {
        if cap > cap_threshold || prices[i] > 1000.0 {
            outliers.push((cap, prices[i], countries[i].clone()));
            println!(
                "Outlier - Market Cap: {:.2}, Price: {:.2}, Country: {}",
                cap, prices[i], countries[i]
            );
        }
    }

 
    let root = BitMapBackend::new("outliers.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Outliers (Market Cap and Price)", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..outliers.len(), 0..*prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() as usize)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(
        outliers.iter().enumerate().map(|(i, &(_cap, price, _))| {
            Circle::new((i, price as usize), 5, ShapeStyle {
                color: RED.to_rgba(),
                filled: true,
                stroke_width: 1,
            })
        }),
    )?;


        
let mut graph = Graph::new();
for (i, country) in countries.iter().enumerate() {
    for (j, other_country) in countries.iter().enumerate() {
        if i != j && country == other_country {
            graph.add_edge(i, j); 
        }
    }
}


let degree_centrality = graph.compute_degrees();
println!("Degree Centrality: {:?}", degree_centrality);


println!("\nU.S. Nodes with Centrality:");
for (&node, &centrality) in &degree_centrality {
    if let Some(label) = countries.get(node) {
        if label == "United States" {
            println!("Node {} (US): Centrality {}", node, centrality);
        }
    }
}


let mut labels = HashMap::new();
for (i, country) in countries.iter().enumerate() {
    labels.insert(i, country.clone());
}


plot_centrality(&degree_centrality, &labels)?;


plot_centrality_no_labels(&degree_centrality)?;

Ok(())
}


pub fn calculate_histogram(data: &[f64], bins: usize) -> Vec<usize> {
    if data.is_empty() || bins == 0 {
        return vec![0; bins]; 
    }

    let max_value = data.iter().cloned().fold(f64::MIN, f64::max);
    let min_value = data.iter().cloned().fold(f64::MAX, f64::min);
    let bin_width = (max_value - min_value) / bins as f64;

    let mut histogram = vec![0; bins];

    for &value in data {
        let bin = if value == max_value {
            bins - 1 
        } else {
            ((value - min_value) / bin_width).floor() as usize
        };
        histogram[bin] += 1;
    }

    histogram
}

pub fn calculate_country_representation(countries: &[String]) -> HashMap<String, usize> {
    let mut country_map = HashMap::new();
    for country in countries {
        *country_map.entry(country.clone()).or_default() += 1;
    }
    country_map
}

pub fn detect_outliers(data: &[f64], threshold: f64) -> Vec<f64> {
    data.iter()
        .cloned()
        .filter(|&value| value > threshold)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_calculate_histogram_non_uniform() {
    let data = vec![0.5, 1.5, 2.0, 2.8, 3.3, 3.7, 4.1, 4.9, 5.0, 5.5];
    let bins = 4;
    let histogram = calculate_histogram(&data, bins);

    
    let total_count: usize = histogram.iter().sum();
    assert_eq!(total_count, data.len());


    println!("Histogram: {:?}", histogram);

   
    assert_eq!(histogram[0], 2); 
    assert_eq!(histogram[1], 2); 
    assert_eq!(histogram[2], 3); 
    assert_eq!(histogram[3], 3); 
}


    #[test]
    fn test_calculate_country_representation() {
        let countries = vec![
            "USA".to_string(),
            "USA".to_string(),
            "Canada".to_string(),
            "India".to_string(),
            "Canada".to_string(),
        ];
        let country_map = calculate_country_representation(&countries);
        assert_eq!(country_map.get("USA"), Some(&2));
        assert_eq!(country_map.get("Canada"), Some(&2));
        assert_eq!(country_map.get("India"), Some(&1));
    }

    #[test]
    fn test_detect_outliers() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let threshold = 35.0;
        let outliers = detect_outliers(&data, threshold);
        assert_eq!(outliers, vec![40.0, 50.0]);
    }
}
