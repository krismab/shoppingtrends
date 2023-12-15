use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::prelude::DiGraphMap;

use crate::Item;
//creates nodes for each purchased item
pub(crate) fn create_nodes(
    graph: &mut DiGraphMap<NodeIndex, ()>,
    items: &[Item],
) -> HashMap<String, NodeIndex> {
    let mut item_nodes: HashMap<String, NodeIndex> = HashMap::new();

    for item in items {
        item_nodes.entry(item.item_purchased.clone())
            .or_insert_with(|| graph.add_node(NodeIndex::new(graph.node_count())));
    }

    item_nodes
}

//creates edges between nodes based on the same category
pub(crate) fn create_edges(
    graph: &mut DiGraphMap<NodeIndex, ()>,
    items: &[Item],
    item_nodes: &HashMap<String, NodeIndex>,
) {
    for item in items {
        let node = item_nodes.get(&item.item_purchased).unwrap();

        // Example: Connect items with the same category
        for other_item in items {
            if item.category == other_item.category && item.item_purchased != other_item.item_purchased {
                let other_node = item_nodes.get(&other_item.item_purchased).unwrap();
                graph.add_edge(*node, *other_node, ());
            }
        }
    }
}

//builds the graph based on the nodes and the edges
pub(crate) fn build_graph(items: &[Item]) -> (DiGraphMap<NodeIndex, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraphMap::new();

    let item_node_mapping = create_nodes(&mut graph, items);
    create_edges(&mut graph, items, &item_node_mapping);

    (graph, item_node_mapping)
}