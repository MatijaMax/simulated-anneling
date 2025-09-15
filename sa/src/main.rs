// MATIJA MAKSIMOVIĆ E2 33/2024

mod shared;
mod parallel;
mod sequential;

use shared::*;
use sequential::SA;
use parallel::run_parallel_sa;

fn main() {
    // Parse input file
    let (cities, distances) = parse_input("../input.txt");
    
    println!("Number of cities: {}", cities.len());
    
    // Sequential SA
    println!("\nSequential SA:");
    let mut sa_seq = SA::new(cities.clone(), distances.clone());
    let best_seq = sa_seq.run();
    println!("Sequential BEST distance: {}", tour_distance(&best_seq, &distances));
    write_tour_to_file("tour_seq.txt", &best_seq, &distances).expect("Failed to write tour_seq.txt");

    // Parallel SA
    println!("\nParallel SA:");
    let best_par = run_parallel_sa(cities.clone(), distances.clone(), 8); 
    println!("Parallel BEST distance: {}", tour_distance(&best_par, &distances));
    write_tour_to_file("tour_par.txt", &best_par, &distances).expect("Failed to write tour_par.txt");
    // BEST tour parallel
    // println!("\nBEST tour parallel:");
    // for city in &best_par {
    //     println!("{} ({})", city.name, city.country);
    // }
}