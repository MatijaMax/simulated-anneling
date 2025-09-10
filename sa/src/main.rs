mod shared;
mod parallel;
mod sequential;

use shared::*;
use sequential::SA;


fn main() {
    // Parse input file
    let (cities, distances) = parse_input("../input.txt");
    
    println!("Number of cities: {}", cities.len());
    
    // Sequential SA
    println!("\nSequential SA:");
    let mut sa_seq = SA::new(cities.clone(), distances.clone());
    let best_seq = sa_seq.run();
    println!("Sequential BEST distance: {}", tour_distance(&best_seq, &distances));

    // BEST tour sequential
    // println!("\nBEST tour sequential:");
    // for city in &best_seq {
    //     println!("{} ({})", city.name, city.country);
    // }

    // Parallel SA
    println!("\nParallel SA:");
    println!("Parallel BEST distance: ");

    // BEST tour parallel
    println!("\nBEST tour parallel:");

}
