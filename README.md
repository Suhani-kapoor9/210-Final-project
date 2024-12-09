# 210-Final-project
This project leverages graph theory and visualization techniques to analyze a dataset of companies. The goal is to examine centrality measures, market cap distributions, global representation, and outliers. Two distinct visualizations—centrality with and without labels—highlight insights, particularly focusing on United States companies.

The code is implemented in Rust, utilizing the `plotters` library for visualizations and a modular design for clarity and scalability. Below, we delve into the methods used, their implementation, and the outcomes observed.
### **Market Cap Distribution**
- **Insight**: The majority of companies have smaller market caps, with a steep decline in frequency for larger caps. This reflects the reality that most firms are small to medium-sized, with a few major players dominating.

### **Global Representation**
- **Insight**: The bar chart highlights the dominance of companies based in the United States, which far surpasses representation from other countries. This emphasizes the economic influence of the U.S.

### **Outliers**
- **Insight**: The outlier chart pinpoints companies with extremely high market caps or stock prices. These outliers likely correspond to major corporations with global impact, such as tech giants or industry leaders.

### **Graph Centrality**
- **Insight**: Centrality analysis reveals that most nodes (companies) have low connectivity, representing limited shared attributes. However, U.S. companies stand out with higher centrality, indicating strong interconnectedness and influence within the dataset.
- **US-Focused Centrality**: Nodes labeled as "United States" with significant centrality demonstrate the central role of U.S.-based companies in the dataset's network structure.

## Conclusion
This project demonstrates how graph theory and data visualization can uncover patterns and anomalies in a dataset. By combining centrality measures with focused visualizations, the analysis highlights the structural relationships and dominance of specific entities within the dataset. The modular codebase ensures extensibility, allowing further exploration of other graph metrics or datasets.


