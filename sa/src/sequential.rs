use crate::shared::{City, DistanceMatrix, tour_distance, shuffle_tour};
use rand::prelude::*;

pub struct SA {
    pub cities: Vec<City>,
    pub distances: DistanceMatrix,
    pub k_max: u32,
    pub k_t: f64,
    pub rng: StdRng,
}

impl SA {
    pub fn new(cities: Vec<City>, distances: DistanceMatrix) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut tour = cities.clone();
        shuffle_tour(&mut tour, &mut rng);

        SA {
            cities: tour,
            distances,
            k_max: 500_000,
            k_t: 500.0,
            rng,
        }
    }

    fn temperature(&self, k: u32) -> f64 {
        self.k_t * (1.0 - k as f64 / self.k_max as f64)
    }

    fn probability(&self, delta_e: i32, t: f64) -> f64 {
        if delta_e < 0 { 1.0 } else { (-delta_e as f64 / t).exp() }
    }

    fn next_permutation(&mut self, tour: &Vec<City>) -> Vec<City> {
        let mut new_tour = tour.clone();
        let len = new_tour.len();
        let i = self.rng.gen_range(0..len);
        let j = self.rng.gen_range(0..len);
        new_tour.swap(i, j);
        new_tour
    }

    pub fn run(&mut self) -> Vec<City> {
        for k in 0..self.k_max {
            let t = self.temperature(k);
            let current_distance = tour_distance(&self.cities, &self.distances) as i32;

            let cities_clone = self.cities.clone();
            let next_tour = self.next_permutation(&cities_clone);
            let next_distance = tour_distance(&next_tour, &self.distances) as i32;

            let delta_e = next_distance - current_distance;
            let prob: f64 = rand::Rng::gen(&mut self.rng);

            if self.probability(delta_e, t) >= prob {
                self.cities = next_tour;
            }

            if k % 50_000 == 0 {
                println!("Iteration: {}, Distance: {}", k, tour_distance(&self.cities, &self.distances));
            }
        }
        self.cities.clone()
    }
}