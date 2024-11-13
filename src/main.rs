pub mod graph;

use graph::StochasticGraph;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng};
use std::time::Instant;

/** psudo code
 * We have 10k nodes
 * Partition to sqrt(N_V) groups, i.e. 100, each w rougly 100 nodes
 * Assign edges between the groups:
 *      
 */

fn worker(part:Vec<usize>, worker_id:usize) -> Vec<usize>{
    println!("[#WORKER {}#] Nodes: {}", worker_id, part.len());

    let mut to_remove:Vec<usize> = Vec::new(); // List of nodes to remove
    let mut graph:Vec<Vec<usize>> = Vec::new();

    // Create adj list  
    // for v1 in part.iter() {
    //     let mut node_list: Vec<usize> = Vec::new();
    //     // let mut degree = 0;
    //     for v2 in part.iter() {
    //         if v1 != v2 && has_edge(*v1, *v2) {
    //             node_list.push(*v2);
    //             // degree += 1;
    //         }
    //     }
    //     graph.push(node_list);
    // }

    print!("Graph:");
    for i in 0..graph.len() {
        print!("{}: {}, ", part[i], graph[i].len());
    }
    println!();

    // Sort graph -- Simple and ensures largest degrees matches first
    // HOWEVER: Does not account for nodes removed *during* matching
    graph.sort_by(|a, b| b.len().cmp(&a.len()));

    // Find matchings
    for i in 0..graph.len() {
        for neighbor in &graph[i] {
            // Have to do lengthy to_remove.contains() because we cannot remove node half way through iterating the list
            if !to_remove.contains(&part[i]) && !to_remove.contains(&neighbor) {
                // Match found!
                to_remove.push(part[i]);
                to_remove.push(neighbor.clone());
            }
        }
    }

    return to_remove;
}

fn main() {
    // Constants
    const N:usize = 100; // Number of partitions
    // const N_V:usize = N * N; // Number of Verticies
    const SEED:&str = "seed"; // Seed used in md5 hashing and rng
    const FILE_NAME:&str = "graphs/sto-log-n.html";

    // Program starts here
    let mut g1: StochasticGraph = StochasticGraph::new(N, SEED);
    let degrees: Vec<usize> = g1.count_degrees(&Vec::new());

    // println!("Starting with N={} workers and N_V={} verticies; seed = {}", N, N_V, SEED);
    // let now = Instant::now();
    // let mut partitions:Vec<Vec<usize>> = vec![Vec::new();N]; // creates 2d vector
    // let mut removed:Vec<usize> = Vec::new();

    // // Assign nodes and count initial degrees
    // let mut degrees_init:Vec<usize> = vec![0;N_V];
    // for node in 0..N_V {
    //     let machine = rand::thread_rng().gen_range(0..N);
    //     partitions[machine].push(node);

    //     for v2 in 0..N_V {
    //         if node != v2 && has_edge(node, v2) { // if edge exists 
    //             degrees_init[node] += 1; // then increment count
    //         }   
    //     }
    // }
    // println!("Initial Degrees: {:?}", degrees_init);
    // println!("Initial count & machine assigning complete. Time elapsed: {:.2?}. Now running algorithm...", now.elapsed());

    // // Run workers
    // for i in 0..N {
    //     // runs worker and adds to_remove (from worker) to removed (global)
    //     removed.append(&mut worker(partitions[i].clone(), i));
    // }
    // println!("Algorithm complete. Time elapsed: {:.2?}. Now counting final degrees...", now.elapsed());

    // // Count degrees 
    // let degrees_after:Vec<usize> = count_degrees(&removed);
    // println!("Ending Degrees: {:?}", degrees_after);

    // Graph
    g1.graph(degrees, FILE_NAME);

    // println!("Program ended. Total time elapsed: {:.2?}.", now.elapsed());
}