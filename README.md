# 210-Final-project
This repository contains the complete implementation and documentation for the 210 Final Project: Graph Analysis and Visualization, developed using Rust. It includes the source code files main.rs (main program logic) and graph.rs (graph construction and analysis module), a dataset file (companies1.csv), and various visualizations such as market cap distribution, global representation, outlier detection, and graph centrality (with and without labels). The project explores company trends, geographical dominance, and network influence using graph theory and visualizations generated with the plotters library. Additionally, the repository features a detailed project write-up (Project Write-Up_ Graph Analysis and Visualization.md) explaining the methodology and insights, alongside the required build files (Cargo.toml and Cargo.lock) for reproducibility.

This project leverages graph theory and visualization techniques to analyze a dataset of companies. The goal is to examine centrality measures, market cap distributions, global representation, and outliers. Two distinct visualizations—centrality with and without labels—highlight insights, particularly focusing on United States companies.

Graph Construction: Nodes represent companies, and edges are formed based on shared attributes like countries.
Market Cap Analysis: Histogram visualization of market cap distribution to identify trends.
Global Representation: Bar chart showing the number of companies per country, emphasizing geographical dominance.
Outlier Detection: Identification of companies with exceptionally high market caps or stock prices.
Degree Centrality: Analysis of node connectivity to measure influence within the dataset's graph structure.
Focused Visualizations:
Centrality without labels for an overview of the network.
Centrality with labels highlighting U.S.-based companies.

This project demonstrates how graph theory and data visualization can uncover patterns and anomalies in a dataset. By combining centrality measures with focused visualizations, the analysis highlights the structural relationships and dominance of specific entities within the dataset. The modular codebase ensures extensibility, allowing further exploration of other graph metrics or datasets.
