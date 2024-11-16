pub mod graph;

use graph::StochasticGraph;
use rand::{rngs::SmallRng, Rng};
use rand_seeder::Seeder;
use std::{env, time::Instant};

/** psudo code
 * We have 10k nodes
 * Partition to sqrt(N_V) groups, i.e. 100, each w rougly 100 nodes
 * Assign edges between the groups:
 *      
 */

fn worker(part:Vec<usize>, worker_id:usize, g:&mut StochasticGraph) -> Vec<usize>{
    println!("[#WORKER {}#] Nodes: {}", worker_id, part.len());

    let mut to_remove:Vec<usize> = Vec::new(); // List of nodes to remove
    let mut graph:Vec<Vec<usize>> = Vec::new();

    // Create adj list  
    for v1 in part.iter() {
        let mut node_list: Vec<usize> = Vec::new();

        for v2 in part.iter() {
            if v1 != v2 && g.has_edge(*v1, *v2) {
                node_list.push(*v2);
            }
        }
        graph.push(node_list);
    }

    // print!("Graph:");
    // for i in 0..graph.len() {
    //     print!("{}: {}, ", part[i], graph[i].len());
    // }
    // println!();

    // Sort graph -- Simple and ensures largest degrees matches first
    // HOWEVER: Does not account for nodes removed *during* matching
    graph.sort_by(|a, b| b.len().cmp(&a.len()));

    // Find matchings
    for i in 0..graph.len() { // I think largest matches generally has lower index here?
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
    // env::set_var("RUST_BACKTRACE", "1"); // Setting env variable for debugging
    
    // Constants ------------------------------------------------------------------
    const N:usize = 1000; // Number of partitions
    const N_V:usize = N * N; // Number of Verticies

    const SEED:&str = "seed"; // Seed used in md5 hashing and rng
    const FILE_NAME:&str = "sto-log-n-1m.html";
    // -----------------------------------------------------------------------------

    let mut rng:SmallRng = Seeder::from(SEED).make_rng(); // create rng from seed

    // Program starts here
    let mut g1: StochasticGraph = StochasticGraph::new(N, SEED.to_string());

    println!("Starting with N={} workers and N_V={} verticies; seed = {}", N, N_V, SEED);
    let now = Instant::now();
    let mut partitions:Vec<Vec<usize>> = vec![Vec::new();N]; // creates a 2d vector
    let mut removed:Vec<usize> = Vec::new();

    // Randonly assign nodes
    for node in 0..N_V {
        let machine = rng.gen_range(0..N);
        partitions[machine].push(node);
    }

    let degrees_init:Vec<usize> = g1.count_degrees(&removed);
    g1.add_graph(degrees_init);

    println!("Initial count & machine assigning complete. Time elapsed: {:.2?}. Now running algorithm...", now.elapsed());

    // Run workers
    for i in 0..N {
        // also adds to_remove (from worker) to removed (global)
        removed.append(&mut worker(partitions[i].clone(), i, &mut g1));
    }

    println!("Algorithm complete. Time elapsed: {:.2?}. Now counting final degrees...", now.elapsed());

    // Count and graph poost-degrees 
    let degrees_after:Vec<usize> = g1.count_degrees(&removed);
    g1.add_graph(degrees_after);

    // write html
    g1.write_html(FILE_NAME);

    // Print total run time
    println!("Program ended. Total time elapsed: {:.2?}.", now.elapsed());
}