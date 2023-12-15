use std::collections::{HashMap, HashSet};
use petgraph::graph::NodeIndex;
use petgraph::prelude::DiGraphMap;

use crate::Item;

// Calculates the degree centrality of each node in the graph.
pub(crate) fn calculate_degree_centrality(graph: &DiGraphMap<NodeIndex, ()>) -> Vec<f64> {
    let num_nodes = graph.node_count() as f64;

    let degrees: Vec<usize> = graph.nodes().map(|node| graph.neighbors(node).count()).collect();

    degrees.iter().map(|&degree| degree as f64 / (num_nodes - 1.0)).collect()
}

// Calculates the seasonal degree centrality of each node in the graph.
pub(crate) fn calculate_seasonal_degree_centrality(
    graph: &DiGraphMap<NodeIndex, ()>,
    items: &[Item],
    item_node_mapping: &HashMap<String, NodeIndex>,
) -> HashMap<String, Vec<f64>>{
    let mut seasonal_centrality: HashMap<String, Vec<f64>> = HashMap::new();

    for season in items.iter().map(|item| &item.season).collect::<HashSet<&String>>() {
        let subgraph_nodes: Vec<NodeIndex> = items
            .iter()
            .filter(|item| &item.season == season)
            .filter_map(|item| item_node_mapping.get(&item.item_purchased))
            .copied()
            .collect();
    

        let num_nodes = subgraph_nodes.len() as f64;
        let mut centrality_scores = Vec::new();

        for node in subgraph_nodes {
            let degree = graph.neighbors(node).count() as f64;
            centrality_scores.push(degree / (num_nodes - 1.0));
        }

        seasonal_centrality.insert(season.clone(), centrality_scores);
    }

    seasonal_centrality
}

