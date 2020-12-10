use std::fs;
use std::env;


fn main() {
    
    // Read in file  *
    // Parse ints from String *
    // File -> Vec<u8> -> String -> Vec<u32> *
    // Iterate sums
    // Track previous attempts to save compute
    // Find 2020
    // Return product of terms
    let target = 2020;
    let filename = env::args().nth(1).unwrap();
    let file_string = String::from_utf8(fs::read(filename).unwrap()).unwrap();
    let inputs : Vec<usize> = file_string
        .split("\n")
        .map(|i| i.parse::<usize>().unwrap_or_default())
        .collect();
    println!("{:?}", inputs);
   
    let mut i : usize = 0;
    while i < inputs.len() - 1 {
        let a = inputs[i];
        let mut o : usize = i + 1;
        while o < inputs.len() {
            let b = inputs[o];
            if a + b == target {
                println!("Solution found: {} * {} = {}", a, b, a * b);
                return;
            }
            o += 1;
        }
        i += 1;
    }

    println!("No solution found");

}
