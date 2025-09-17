// MATIJA MAKSIMOVIĆ E2 33/2024

mod shared;
mod parallel;
mod sequential;

use shared::*;
use sequential::SA;
use parallel::run_parallel_sa;
use std::time::Instant;

fn main() {
    let (cities, distances) = parse_input("../input.txt");
    println!("Number of cities: {}", cities.len());

    let ns = [8, 12, 16, 20];
    for &n in &ns {
        let subset_cities = cities[..n].to_vec();

        // Sequential
        let mut sa_seq = SA::new(subset_cities.clone(), distances.clone());
        let start_seq = Instant::now();
        let best_seq = sa_seq.run();
        let duration_seq = start_seq.elapsed().as_secs_f64();
        println!("n={}: Sequential {:.3}s", n, duration_seq);

        // Parallel
        let start_par = Instant::now();
        let best_par = run_parallel_sa(subset_cities.clone(), distances.clone(), 8); 
        let duration_par = start_par.elapsed().as_secs_f64();
        println!("n={}: Parallel {:.3}s", n, duration_par);

        // Write tours and distances to tour_seq.txt and tour_par.txt
        write_tour_to_file(&format!("tour_seq_n_{}.txt", n), &best_seq, &distances)
            .expect("Failed to write sequential tour");
        write_tour_to_file(&format!("tour_par_n_{}.txt", n), &best_par, &distances)
            .expect("Failed to write parallel tour");

        // Performance summary for n
        let summary_filename = format!("performance_n_{}.txt", n);
        let summary_content = format!(
            "n = {}\nSequential time: {:.3} s\nParallel time: {:.3} s\nSpeedup: {:.2}x\nSequential distance: {}\nParallel distance: {}\n",
            n,
            duration_seq,
            duration_par,
            duration_seq / duration_par,
            tour_distance(&best_seq, &distances),
            tour_distance(&best_par, &distances)
        );
        std::fs::write(&summary_filename, summary_content).expect("Failed to write performance summary");
    }
}
