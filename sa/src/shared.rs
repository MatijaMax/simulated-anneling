use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct City {
    pub name: String,
    pub country: String,
}

pub type DistanceMatrix = HashMap<(City, City), u32>;

/// Distance calculation
pub fn tour_distance(tour: &[City], distances: &DistanceMatrix) -> u32 {
    let mut sum = 0;
    for i in 0..tour.len() - 1 {
        sum += distances
            .get(&(tour[i].clone(), tour[i + 1].clone()))
            .or_else(|| distances.get(&(tour[i + 1].clone(), tour[i].clone())))
            .unwrap();
    }
    sum += distances
        .get(&(tour[tour.len() - 1].clone(), tour[0].clone()))
        .or_else(|| distances.get(&(tour[0].clone(), tour[tour.len() - 1].clone())))
        .unwrap();
    sum
}

/// Initial shuffle 
pub fn shuffle_tour(tour: &mut Vec<City>, rng: &mut rand::prelude::StdRng) {
    use rand::seq::SliceRandom;
    tour.shuffle(rng);
}

/// Input.txt parsing
pub fn parse_input(file_path: &str) -> (Vec<City>, DistanceMatrix) {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    let file = File::open(file_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut cities_map = HashMap::new();
    let mut distances = DistanceMatrix::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(',').collect();
        let city1 = City { name: parts[0].to_string(), country: parts[1].to_string() };
        let city2 = City { name: parts[2].to_string(), country: parts[3].to_string() };
        let dist: u32 = parts[4].parse().unwrap();

        cities_map.insert(city1.clone(), ());
        cities_map.insert(city2.clone(), ());
        distances.insert((city1, city2), dist);
    }

    let cities = cities_map.into_keys().collect::<Vec<_>>();
    (cities, distances)
}
