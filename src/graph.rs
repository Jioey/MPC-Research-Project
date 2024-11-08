use rand::prelude::*;
use rand_seeder::Seeder;

pub struct StochasticGraph {
    n:usize,
    n_v:usize,
    seed:String,
    prob_matrix:Vec<Vec<f32>>
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

    /// Generates a n by n probability matrix
    /// Values are randomly chosen from the supported probabilities using the seed
    /// Note: Only upper half (including diagonal) is populate for efficiency sake
    fn generate_prob_matrix(n:usize, seed:&str) -> Vec<Vec<f32>> {
        let supported_probs: Vec<f32> = vec![0.1, 0.2, 0.25, 0.33, 0.5];

        // Generation
        let mut prob_matrix: Vec<Vec<f32>> = vec![vec![0.0; n]; n];

        let mut rng:SmallRng = Seeder::from(seed).make_rng();
        
        for i in 0..n {
            for j in i..n {
                let p = supported_probs.choose(&mut rng).unwrap();
                prob_matrix[i][j] = *p; // only populating upper half (including diagonal)
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
        
        return hash_as_int % Self::prob_to_modded(p) == 0;
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
}