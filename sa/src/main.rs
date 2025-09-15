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

    // Parallel SA
    println!("\nParallel SA:");
    let best_par = run_parallel_sa(cities.clone(), distances.clone(), 8); 
    println!("Parallel BEST distance: {}", tour_distance(&best_par, &distances));

    // BEST tour parallel
    // println!("\nBEST tour parallel:");
    // for city in &best_par {
    //     println!("{} ({})", city.name, city.country);
    // }
}