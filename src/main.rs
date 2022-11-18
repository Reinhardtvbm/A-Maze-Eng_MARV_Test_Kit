// mod components {
//     pub mod colour;
//     pub mod comm_port;
// }

// mod subsystems {
//     pub mod mdps;
//     //pub mod snc;
//     pub mod ss;
//     mod snc_components {
//         pub mod navcon;
//     }
// }

use maze::graph::Graph;

mod maze {
    pub mod graph;
    pub mod maze;
}

fn main() {
    let mut new_graph: Graph<u8> = Graph::new();

    new_graph.add_node(1);
    println!("==================================");
    new_graph.add_node(2);
    println!("==================================");
    new_graph.add_node(2);
    println!("==================================");
    new_graph.add_node(2);
    println!("==================================");
    new_graph.add_node(2);
    println!("==================================");
    new_graph.add_node(2);

    println!("{}", new_graph.get_node(1).unwrap());
}
