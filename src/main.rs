// Rust is Data Oriented?! (Contrary to OOP)
const N:u32 = 100;
const N_V:u32 = N * N;
const SEED:&str = "seed";

fn has_edge(v1:u32, v2:u32) -> bool {
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

fn worker() {
    
}

fn main() {
    println!("Starting with N={} workers and N_V={} verticies; seed = {}", N, N_V, SEED);

    println!("{}", has_edge(2, 3));
}