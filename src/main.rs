
use std::{error::Error, collections::HashMap};
use csv::Reader; 
use petgraph::graph::NodeIndex;

mod graph;
mod centrality;

#[derive(Debug, Clone, PartialEq,Eq, Hash)]
//the struct for the csv file
struct Item {
    customer_id: usize,
    age: usize,
    gender: bool,
    item_purchased: String,
    category: String,
    purchase_amount: usize,
    location: String,
    size: String,
    color: String,
    season: String,
    review_rating: usize,
    subscription_status: bool,
    shipping_type: String,
    discount_applied: bool,
    promo_code_used: bool,
    previous_purchases: usize,
    payment_method: String,
    preferred_payment_method: String,
    frequency_of_purchases: String,
    edges:Vec<String>,
}


//reads the csv file and returns a vector of items
fn read_csv(file_path: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let _headers = reader.headers()?.clone(); 

    let data: Vec<Item> = reader
        .records()
        .filter_map(|result| {
            result.ok().and_then(|record| {

                Some(Item {
                    customer_id: record[0].parse().unwrap_or_default(),
                    age: record[1].parse().unwrap_or_default(),
                    gender: record[2].parse().unwrap_or(false),
                    item_purchased: record[3].to_string(),
                    category: record[4].to_string(),
                    purchase_amount: record[5].parse().unwrap_or_default(),
                    location: record[6].to_string(),
                    size: record[7].to_string(),
                    color: record[8].to_string(),
                    season: record[9].to_string(),
                    review_rating: record[10].parse().unwrap_or_default(),
                    subscription_status: record[11].parse().unwrap_or_default(),
                    shipping_type: record[12].to_string(),
                    discount_applied: record[13].parse().unwrap_or_default(),
                    promo_code_used: record[14].parse().unwrap_or_default(),
                    previous_purchases: record[15].parse().unwrap_or_default(),
                    payment_method: record[16].to_string(),
                    preferred_payment_method: record[17].to_string(),
                    frequency_of_purchases:record[18].to_string(),
                    edges: Vec::new(),
                })
            })
        })
        .collect();
    
     Ok(data)
}

fn main() {
    match read_csv("/Users/krisma/Desktop/210project/shopping_trends.csv") {
        Ok(items) => {
            let (graph, item_node_mapping) = graph::build_graph(&items);

            let degree_centrality = centrality::calculate_degree_centrality(&graph);

            // Create a reverse mapping from NodeIndex to item name
            let reverse_mapping: HashMap<NodeIndex, String> = item_node_mapping
                .iter()
                .map(|(item, &node)| (node, item.clone()))
                .collect();

            // Print the degree centrality along with the item name
            for node in graph.nodes() {
                if let Some(item_name) = reverse_mapping.get(&node) {
                    let centrality = degree_centrality[node.index()];
                    println!("Item '{}': Degree Centrality: {:.4}", item_name, centrality);
                }
            }

            let seasonal_centrality = centrality::calculate_seasonal_degree_centrality(&graph, &items, &item_node_mapping);

            // Print the seasonal degree centrality for each node with item names
            for (season, centrality_scores) in seasonal_centrality.iter() {
                println!("Season {}:", season);
                for (node, centrality) in graph.nodes().zip(centrality_scores.iter()) {
                    if let Some(item_name) = reverse_mapping.get(&node) {
                        println!("  Item '{}': Seasonal Degree Centrality: {:.4}", item_name, centrality);
                    }
                }
            }
        }
        Err(e) => println!("Error reading CSV file: {:?}", e),
    }
}



//test module 
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_items() -> Vec<Item> {
        vec![
            Item {
                customer_id: 1,
                age: 30,
                gender: true,
                item_purchased: "Shirt".to_string(),
                category: "Clothing".to_string(),
                purchase_amount: 100,
                location: "Hawaii".to_string(),
                size: "M".to_string(),
                color: "Grey".to_string(),
                season: "Spring".to_string(),
                review_rating: 3,
                subscription_status: true,
                shipping_type: "Express".to_string(),
                discount_applied: false,
                promo_code_used: false,
                previous_purchases: 3,
                payment_method: "Venmo".to_string(),
                preferred_payment_method: "Credit Card".to_string(),
                frequency_of_purchases: "Every 3 Months".to_string(),
                edges: Vec::new(),
            

            },
            Item {
                customer_id: 2,
                age: 25,
                gender: false,
                item_purchased: "Pants".to_string(),
                category: "Clothing".to_string(),
                purchase_amount: 150,
                location: "New York".to_string(),
                size: "L".to_string(),
                color: "Black".to_string(),
                season: "Winter".to_string(),
                review_rating: 4,
                subscription_status: false,
                shipping_type: "Standard".to_string(),
                discount_applied: true,
                promo_code_used: true,
                previous_purchases: 5,
                payment_method: "Credit Card".to_string(),
                preferred_payment_method: "Credit Card".to_string(),
                frequency_of_purchases: "Once a Year".to_string(),
                edges:Vec::new(),
            },
            // Add more test items as needed
        ]
    }

    #[test]
    fn test_create_nodes() {
        let items = create_test_items();
        let mut graph = petgraph::graphmap::DiGraphMap::new();
        let nodes = graph::create_nodes(&mut graph, &items);

        assert_eq!(nodes.len(), items.len());
        assert!(nodes.contains_key("Shirt"));
        assert!(nodes.contains_key("Pants"));
    }
    #[test]
    fn test_read_csv() {
        let file_path = "/Users/krisma/Desktop/210project/shopping_trends.csv"; let data = read_csv(file_path).unwrap();
        assert_eq!(data.len(), 3901); // Num of rows in CSV file
    }

    #[test]
    fn test_create_edges() {
        let items = create_test_items();
        let mut graph = petgraph::graphmap::DiGraphMap::new();
        let item_nodes = graph::create_nodes(&mut graph, &items);

        graph::create_edges(&mut graph, &items, &item_nodes);

        let shirt_node = item_nodes.get("Shirt").unwrap();
        let pants_node = item_nodes.get("Pants").unwrap();

        assert!(graph.contains_edge(*shirt_node, *pants_node));
    }


    #[test]
    fn test_centrality() {
            let items = create_test_items();
            let (graph, item_node_mapping) = graph::build_graph(&items);
    
            let degree_centrality = centrality::calculate_degree_centrality(&graph);
    
            let shirt_node = item_node_mapping.get("Shirt").unwrap();
            let pants_node = item_node_mapping.get("Pants").unwrap();
    
            let shirt_centrality = degree_centrality[shirt_node.index()];
            let pants_centrality = degree_centrality[pants_node.index()];
    
            //'Shirt' and 'Pants' are the only two items and connected, 
            // their centrality should be 1/(2-1) = 1.0
            assert_eq!(shirt_centrality, 1.0);
            assert_eq!(pants_centrality, 1.0);
    }
    

}

