use std::{fs::{self, File}, path::Path, result};

use rand::rngs::SmallRng;
use rand_seeder::Seeder;
use plotly::{common::Title, layout::{Axis, BarMode}, Bar, Layout, Plot};

pub struct StochasticGraph {
    n:usize,
    n_v:usize,
    seed:String,
    rng:SmallRng,
    prob_matrix:Vec<Vec<u128>>,
    plot_html:String
}

// Implicit implementations:
//   Each group is numbers 0...99, 100...199, 200...299, ...
//   High degree groups are nodes from index 0..n out of n_v
//   Group ID = Node Index // n (integer division)
//      Use Group ID in MD5 to check if edge exist

impl StochasticGraph {
    /// Creates new Stochastic Graph instance
    /// n = sqrt(N_V) where N_V is the total number of verticies
    pub fn new(n: usize, seed:String) -> Self {
        let n_v: usize = n* n;
        let plot_html = String::new();

        // Generate the probability matrix
        let prob_matrix = StochasticGraph::generate_prob_matrix(n);

        // Printing Prob Matrix
        // for row in prob_matrix.iter() {
        //     for &prob in row.iter() {
        //         print!("{} ", prob);
        //     }
        //     println!();
        // }

        // Create rng
        let rng:SmallRng = Seeder::from(&seed).make_rng();

        // Creating the instance and returning
        StochasticGraph {
            n,
            n_v,
            seed,
            rng,
            prob_matrix,
            plot_html
        }
    }

    /// Generates a n by n probability matrix, where n is the number of groups
    /// Note: Only upper half (including diagonal) is populate for efficiency sake
    fn generate_prob_matrix(n:usize) -> Vec<Vec<u128>> {
        let mut prob_matrix: Vec<Vec<u128>> = vec![vec![0; n]; n];

        // Assign degree probabilities 
        let log_n = (n as f64).log2() as u128;
        let log_range: Vec<u128> = (1..log_n+1).collect();

        let mut k:usize = 0; // Counter for log_range
        let mut p: u128 = 0;
        for i in 0..n {
            for j in i..n {
                // Handling overflow when calculating p
                if i >= 128 { // When i >= 128 it gaurentees overflow
                    p = std::u128::MAX;
                } else {
                    // Safe multiplication
                    p = match log_range[k].checked_mul((2 as u128).pow(i as u32)) {
                        Some(result) => result, // if success, use the value
                        None => std::u128::MAX, // if overflow, set p to max u128
                    };
                }

                // Assign p
                // only populating upper half (including diagonal) for efficiency
                prob_matrix[i][j] = p; 

                // Increment k
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
    pub fn add_graph(&mut self, degrees:Vec<usize>) {
        println!("[#Analysis#] Adding graph...");

        // Create bins
        let mut bins:Vec<usize> = Vec::new();
        let mut bin:usize = self.n_v / 2;
        while bin >= 1 {
            bins.push(bin);
            bin = bin / 2;
        }

        // Manually count distributions
        // println!("[#Analysis#] Counting distribution...");
        let bin_size = bins.len();
        let mut count:Vec<usize> = vec![0; bin_size+1];
        for i in 0..self.n_v {
            // count initial by bins
            if degrees[i] == 0 {
                count[bin_size] += 1;
            } else {
                for j in 0..bin_size {
                    if degrees[i] > bins[j] {
                        count[j] += 1;
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

        // convert bin numbers to axis ticks
        let mut bin_labels:Vec<String> = Vec::new();
        for b in bins {
            let mut l = ">".to_owned();
            l.push_str(&b.to_string());
            bin_labels.push(l);
        }
        bin_labels.push("0".to_owned());
        println!("Bin labels: {:?}", bin_labels);

        // adding counted distribution
        let trace1 = Bar::new(bin_labels.clone(), count)
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
        
        // Plot
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(trace1);

        // Add to plot html
        self.plot_html.push_str(&plot.to_html());
        self.plot_html.push_str("\n");   
    }

    pub fn write_html(&mut self, file_name:&str) {
        let file:String = String::from("/home/jioey_z/MPC-Research-Project/graphs/") + file_name;

        print!("File ");
        // Check if the file exists (not tested)
        if !Path::new(&file).exists() {
            // If it doesn't exist, create it
            let _ = File::create(&file);
            print!("created (at {}) and ", &file);
        }

        fs::write(file, &self.plot_html).expect("Unable to write file");
        print!("modified \n");
    }
}