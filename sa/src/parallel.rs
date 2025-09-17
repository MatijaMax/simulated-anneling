use crate::shared::{City, DistanceMatrix, tour_distance};
use crate::sequential::SA;
use rayon::prelude::*;
use rand::prelude::*;

pub fn run_parallel_sa(cities: Vec<City>, distances: DistanceMatrix, num_runs: usize) -> Vec<City> {
    let results: Vec<(Vec<City>, u32)> = (0..num_runs)
        .into_par_iter()
        .map(|_| {
            let seed: u64 = rand::thread_rng().gen();
            let mut sa = SA {
                cities: cities.clone(),
                distances: distances.clone(),
                k_max: 50000,
                k_t: 5000.0,
                rng: rand::rngs::StdRng::seed_from_u64(seed),
            };
            let tour = sa.run();
            let dist = tour_distance(&tour, &sa.distances);
            (tour, dist)
        })
        .collect();

    results
        .into_iter()
        .min_by_key(|(_, dist)| *dist)
        .map(|(tour, _)| tour)
        .expect("No results from parallel SA")
}