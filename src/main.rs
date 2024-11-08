pub mod graph;

use graph::StochasticGraph;
use plotly::{common::Title, layout::{Axis, BarMode}, Bar, Layout, Plot};
use rand::{rngs::SmallRng, seq::SliceRandom, Rng};
use std::time::Instant;

// Rust is Data Oriented?! (Contrary to OOP)
const N:usize = 100; // Number of partitions
const N_V:usize = N * N; // Number of Verticies
const SEED:&str = "seed"; // Seed used in md5 hashing and rng
const FILE_NAME:&str = "graphs/temp.html";

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
    let mut g1 = StochasticGraph::new(N, SEED);
    let degrees = g1.count_degrees(&Vec::new());

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

    // Graph!!
    // Manually count distributions
    // create bins
    let mut bins:Vec<usize> = Vec::new();
    let mut bin:usize = N_V / 2;
    while bin >= 1 {
        bins.push(bin);
        bin = bin / 2;
    }
    
    println!("[#Analysis#] Counting distribution...");
    let bin_size = bins.len();
    let mut count_init:Vec<usize> = vec![0; bin_size+1];
    // let mut count_after:Vec<usize> = vec![0; bin_size+1];
    for i in 0..N_V {
        // count initial by bins
        if degrees[i] == 0 {
            count_init[bin_size] += 1;
        } else {
            for j in 0..bin_size {
                if degrees[i] > bins[j] {
                    count_init[j] += 1;
                    break;
                }
            }
        }
    }

    //     // count resulting by bins (this feels so inefficient....)
    //     if degrees_after[i] == 0 {
    //         count_after[bin_size] += 1;
    //     } else {
    //         for j in 0..bin_size {
    //             if degrees_after[i] > bins[j] {
    //                 count_after[j] += 1;
    //                 break;
    //             }
    //         }
    //     }
    // }

    println!("[#Analysis#] Graphing to {}...", FILE_NAME);
    // convert bin numbers to axis ticks
    let mut bin_labels:Vec<String> = Vec::new();
    for b in bins {
        let mut l = ">".to_owned();
        l.push_str(&b.to_string());
        bin_labels.push(l);
    }
    bin_labels.push("0".to_owned());
    println!("Bin labels: {:?}", bin_labels);
    // adding initial distribution
    let trace1 = Bar::new(bin_labels.clone(), count_init)
        .name("Before")
        .opacity(0.75);

    // // adding resulting distribution
    // let trace2 = Bar::new(bin_labels.clone(), count_after)
    //     .name("After")
    //     .opacity(0.75);

    // configuring layout
    let layout = Layout::new()
        .title(Title::with_text("Title"))
        .x_axis(Axis::new().title(Title::with_text("Degrees")))
        .y_axis(Axis::new().title(Title::with_text("Count")))
        .bar_mode(BarMode::Overlay)
        .bar_gap(0.05)
        .bar_group_gap(0.2);
    
    // plots graph (to html file, -> open in browser)
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    // plot.add_trace(trace2);
    plot.write_html(FILE_NAME);

    // println!("Program ended. Total time elapsed: {:.2?}.", now.elapsed());
}