use crate::shared::{City, DistanceMatrix, tour_distance};
use crate::sequential::SA;
use rayon::prelude::*;
use rand::SeedableRng;

pub fn run_parallel_sa(cities: Vec<City>, distances: DistanceMatrix, num_runs: usize) -> Vec<City> {
    let results: Vec<(Vec<City>, u32)> = (0..num_runs)
        .into_par_iter()
        .map(|i| {
            let mut sa = SA {
                cities: cities.clone(),
                distances: distances.clone(),
                k_max: 500_000,
                k_t: 500.0,
                rng: rand::rngs::StdRng::seed_from_u64(i as u64 + 1),
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