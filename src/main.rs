use rand::Rng;

// Rust is Data Oriented?! (Contrary to OOP)
const N:usize = 100; // Number of partitions
const N_V:usize = N * N; // Number of Verticies
const SEED:&str = "seed"; // Seed used in md5 hashing

fn has_edge(v1:usize, v2:usize) -> bool {
    // The input string you want to hash
    let input = format!("{}{}{}", SEED, v1, v2);

    // Generate MD5 hash
    let hash = md5::compute(input);
    
    // Convert the MD5 hash (16-byte array) into a 128-bit integer
    let hash_as_int = u128::from_be_bytes(hash.0);
    
    // Print the integer
    println!("MD5 hash as integer: {}", hash_as_int);
    return hash_as_int % 2 == 0;
}

fn worker(part:Vec<usize>) -> Vec<usize>{
    // 好像都不用adj list了？？
    let mut to_remove:Vec<usize> = Vec::new(); // List of nodes to remove

    // TODO: Create adj list    

    // TODO: Sort graph? -- Genius! This essentially ensure largest degrees matches first

    for v1 in part.iter() {
        for v2 in part.iter() {
            // Have to do lengthy .contains() because we cannot remove node half way through iterating the list
            if v1 != v2 && 
            !to_remove.contains(v1) &&
            !to_remove.contains(v2) &&
            has_edge(*v1, *v2) {
                // Match found!
                to_remove.push(*v1);
                to_remove.push(*v2);
            }
        }
    }

    return to_remove;
}

fn main() {
    println!("Starting with N={} workers and N_V={} verticies; seed = {}", N, N_V, SEED);
    let mut partitions:Vec<Vec<usize>> = vec![Vec::new();N]; // creates 2d vector
    let mut removed:Vec<usize> = Vec::new();

    // Assign nodes
    for node in 0..N_V {
        let machine = rand::thread_rng().gen_range(0..N);
        partitions[machine].push(node);
    }

    // Run workers
    for i in 0..N {
        // runs worker and adds to_remove (from worker) to removed (global)
        removed.append(&mut worker(partitions[i].clone()));
    }

    // Count degrees (both before and after)
    let mut degrees_init:Vec<usize> = vec![0;N];
    let mut degrees_after:Vec<usize> = Vec::new();
    for v1 in 0..N_V { // for every pair
        for v2 in 0..N_V {
            if v1 != v2 &&
            has_edge(v1, v2) { // if edge exists
                degrees_init[v1] += 1; // increment initial
                if removed.contains(&v1) &&
                !removed.contains(&v2) { // if node not removed
                    degrees_after[v1] += 1; // then also increment this
                }
            }   
        }
    }




    

    println!("{}", partitions[2][0]);
}