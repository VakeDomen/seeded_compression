use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::thread::{self, spawn};
use std::time::Instant;
use std::collections::HashMap;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::{WeightedIndex, Distribution};

fn read_file_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    // Open the file and read its contents into a byte vector
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn calculate_byte_frequencies(file_bytes: &[u8]) -> HashMap<u8, usize> {
    let mut frequency_map: HashMap<u8, usize> = HashMap::new();

    // Count the occurrences of each byte in the file
    for &byte in file_bytes {
        *frequency_map.entry(byte).or_insert(0) += 1;
    }

    frequency_map
}

fn generate_random_bytes_until_match(file_bytes: &[u8], seed: u64, frequency_map: HashMap<u8, usize>) {
    // Create a seeded RNG
    let mut rng = StdRng::seed_from_u64(seed);
    let timer_start = Instant::now();
    
    let mut generated_bytes: Vec<u8> = Vec::new();
    let mut offset = 0;

    // Calculate byte frequencies in the file

    // Convert the frequency map into weighted distribution for random byte generation
    let (bytes, weights): (Vec<u8>, Vec<usize>) = frequency_map.into_iter().unzip();
    let dist = WeightedIndex::new(&weights).unwrap();

    // Continue generating random bytes until the generated sequence matches the file's content
    loop {
        let random_byte = bytes[dist.sample(&mut rng)];
        generated_bytes.push(random_byte);

        // Check if the generated sequence matches the file's content
        if generated_bytes.ends_with(file_bytes) {
            let timer_end = Instant::now();
            println!("Matching sequence found after generating {} bytes!", generated_bytes.len());
            println!("Offset: {}", offset);
            println!("Seed: {}", seed);
            println!("Time needed: {:?}", timer_end.duration_since(timer_start));
            break;
        }

        // To prevent the generated_bytes vector from growing indefinitely, we can limit its size
        // to the size of the file. If it doesn't match, we can remove the oldest byte.
        if generated_bytes.len() > file_bytes.len() {
            generated_bytes.remove(0);
            offset += 1;
        }
    }
}

fn main() -> io::Result<()> {
    // File path and seed can be provided as command-line arguments or hardcoded for now
    
    let threads = env::var("THREADS");
    let threads = match threads {
        Ok(t) => match t.parse() {
            Ok(t) => t,
            Err(e) => 4,
        },
        Err(e) => 4,
    };
    
    let file_path = env::var("FILE_PATH");
    let file_path = match file_path {
        Ok(t) => t,
        Err(e) => "/bin/chmod".to_string(),
    };

    // Read the file into a byte vector
    let file_bytes = read_file_bytes(file_path.as_str())?;
    println!("File: {}", file_path);
    println!("Bytes to match: {}", file_bytes.len());

    let frequency_map = calculate_byte_frequencies(&file_bytes);


    // Start generating random bytes until the sequence matches the file

    let mut thread_handles = vec![];
    let mut rng =rand::thread_rng();
    let starting_seed = rng.gen::<u64>();

    for seed in starting_seed..(starting_seed + threads) {
        let dist = frequency_map.clone();
        let bytes = file_bytes.clone();
        println!("Matching seed: {}", seed);
        thread_handles.push(thread::spawn(move || generate_random_bytes_until_match(&bytes, seed, dist)));
        
    }


    for t in thread_handles {
        let _  = t.join();
    }

    Ok(())
}
