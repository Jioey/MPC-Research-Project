// use rand::prelude::*;
use rand_seeder::Seeder;
use plotly::{common::Title, layout::{Axis, BarMode}, Bar, Layout, Plot};

pub struct StochasticGraph {
    n:usize,
    n_v:usize,
    seed:String,
    prob_matrix:Vec<Vec<u128>>
}

// Implicit implementations:
//   Each group is numbers 0...99, 100...199, 200...299, ...
//   High degree groups are nodes from index 0..n out of n_v
//   Group ID = Node Index // n (integer division)
//      Use Group ID in MD5 to check if edge exist

impl StochasticGraph {
    /// Creates new Stochastic Graph instance
    /// n = sqrt(N_V) where N_V is the total number of verticies
    pub fn new(n: usize, seed: &str) -> Self {
        let n_v: usize = n* n;

        // Generate the probability matrix
        let prob_matrix = StochasticGraph::generate_prob_matrix(n, seed);

        // Printing spec (printed before create instance because of borrowing issue)
        println!("Successfull generated graph with {} nodes and probability matrix:", n_v);
        for row in prob_matrix.iter() {
            for &prob in row.iter() {
                print!("{} ", prob);
            }
            println!();
        }

        // Creating the instance and returning
        StochasticGraph {
            n,
            n_v,
            seed: seed.to_string(),
            prob_matrix
        }
    }

    /// Generates a n by n probability matrix, where n is the number of groups
    /// Values are randomly chosen from the supported probabilities using the seed
    /// Note: Only upper half (including diagonal) is populate for efficiency sake
    fn generate_prob_matrix(n:usize, seed:&str) -> Vec<Vec<u128>> {
        // Supported probabilities in prob_to_modded()
        // let supported_probs_low: Vec<f32> = vec![0.1, 0.2]; // other p supported: 0.25, 0.33
        // let supported_probs_high: Vec<f32> = vec![0.5, 1.0];
        
        // let num_high_degree_groups = n / 3;

        // Generation
        let mut prob_matrix: Vec<Vec<u128>> = vec![vec![0; n]; n];

        // let mut rng:SmallRng = Seeder::from(seed).make_rng();
        
        // Assign first 1/3 groups with high degree probabilities
        // for i in 0..num_high_degree_groups {
        //     for j in i..n {
        //         let p = supported_probs_high.choose(&mut rng).unwrap();
        //         prob_matrix[i][j] = *p; // only populating upper half (including diagonal)
        //     }
        // }
        
        // // Assign remaining nodes with low degree probabilities 
        // for i in num_high_degree_groups..n {
        //     for j in i..n {
        //         let p = supported_probs_low.choose(&mut rng).unwrap();
        //         prob_matrix[i][j] = *p; 
        //     }
        // }

        // Assign degree probabilities 
        let log_n = (n as f64).log2() as u128;
        println!("Log n = {}, where n is {}", log_n, n);
        let log_range: Vec<u128> = (1..log_n+1).collect();
        let mut k:usize = 0;
        for i in 0..n {
            for j in i..n {
                let p = log_range[k] * (2 as u128).pow(i as u32); // NOTE: p is not probability, it represents 1/p probability
                prob_matrix[i][j] = p; // only populating upper half (including diagonal)

                k += 1;
                k = k % (log_n as usize);
            }
        }

        prob_matrix
    }

    /// Gets Group ID of node v
    pub fn get_group_id(&mut self, v:usize) -> usize {
        return v / self.n;
    }

    /// Gets the number for hash to be modded by based on the probability p
    fn prob_to_modded(p:f32) -> u128 {
        match p {
            0.1=>10,
            0.2=>5,
            0.25=>4,
            0.33=>3,
            0.5=>2,
            1.0=>1,
            _=>{println!("[WARNING] Not supported probability p={}", p);return 0},
        }
    }

    /// Check if an edge exists between v1 and v2
    pub fn has_edge(&mut self, v1:usize, v2:usize) -> bool {
        // The input string you want to hash
        let input = format!("{}{}{}", self.seed, v1, v2);

        // Generate MD5 hash
        let hash = md5::compute(input);
        
        // Convert the MD5 hash (16-byte array) into a 128-bit integer
        let hash_as_int = u128::from_be_bytes(hash.0);
        
        // Print the integer
        // println!("MD5 hash as integer: {}", hash_as_int);

        // Find which group nodes belong to
        let g1 = self.get_group_id(v1);
        let g2 = self.get_group_id(v2);

        // Get probability
        let p;
        if g1 < g2 {
            p = self.prob_matrix[g1][g2];
        } else {
            p = self.prob_matrix[g2][g1];
        }
        
        return hash_as_int % (p as u128) == 0;
    }

    /// Counts the degrees of each node in the graph
    /// Note: Possible to only count outbound edges due to the non-commutativeness of hashing
    pub fn count_degrees(&mut self, removed:&Vec<usize>) -> Vec<usize> {
        let mut degrees:Vec<usize> = vec![0;self.n_v];
        for v1 in 0..self.n_v { // for every pair
            for v2 in 0..self.n_v {
                if v1 != v2 && self.has_edge(v1, v2) && !removed.contains(&v1) && !removed.contains(&v2){ // if edge exists and not removed
                    degrees[v1] += 1; // then increment count
                }   
            }
        }

        degrees
    }

    /// Graph!! Involves counting the distribution, then plotting it using the Plotly package
    pub fn graph(&mut self, degrees:Vec<usize>, file_name:&str) {
        // Manually count distributions
        // create bins
        let mut bins:Vec<usize> = Vec::new();
        let mut bin:usize = self.n_v / 2;
        while bin >= 1 {
            bins.push(bin);
            bin = bin / 2;
        }
        
        println!("[#Analysis#] Counting distribution...");
        let bin_size = bins.len();
        let mut count_init:Vec<usize> = vec![0; bin_size+1];
        // let mut count_after:Vec<usize> = vec![0; bin_size+1];
        for i in 0..self.n_v {
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

        println!("[#Analysis#] Graphing to {}...", file_name);
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
        plot.write_html(file_name);
    }
}