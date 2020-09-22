use crate::city;
use crate::path;
use std::fmt;
use rand::Rng;

#[derive(Debug)]
pub struct Ant {
    #[allow(dead_code)]
    pub tour : Vec<*const path::Path>,
    nest : *const city::City
}

// Pair for the probability of choose the path
struct ProbPath (f32, *const path::Path, *const city::City);

impl Ant {
    pub fn new (nest : &city::City) -> Ant {
        Ant {nest, tour : Vec::new()}
    }

    // Build a tour from all paths in graph
    pub fn build_tour (&mut self, graph : &Vec::<path::Path>, world : &Vec::<city::City>) {
        let world = world.clone();
        let mut actual = unsafe { &*self.nest.clone() };
        let mut cities_to_find : Vec<city::City> = world.into_iter().filter(|x| *x != actual.clone() ).collect::<Vec<city::City>>().clone();
        while !cities_to_find.is_empty() {
            // Get paths where the actual city is from.
            let mut possible_paths : Vec::<&path::Path> = graph.into_iter().filter(|x| *x.get_from_city() == actual.clone()).collect::<Vec<&path::Path>>().clone();
            let tour = self.tour.clone();
            let searched_cities = tour.into_iter().map(|x| unsafe { (*x).get_from_city() }).collect::<Vec<&city::City>>();
            // Remove paths that have a visited city.
            for path in possible_paths.clone() {
                let to_city = path.get_to_city();
                let is_in_cities = searched_cities.iter().position(|x| **x == (*to_city).clone());
                match is_in_cities {
                    Some(_) => self.remove_path(&mut possible_paths, &path),
                    _ => continue,
                }
            }
            if possible_paths.len() == 0 {
                break;
            }
            // There is only a path avaible
            if possible_paths.len() == 1 {
                let final_path = possible_paths[0];
                self.tour.push(final_path);
                actual = &(final_path).get_to_city();
                cities_to_find.clear();
                // Path between nest and final city
                let return_path = graph.iter().find(|x| unsafe { (*x).get_to_city().clone() == (*self.nest).clone() && (*x).get_from_city() == actual}).unwrap();
                self.tour.push(return_path);
                break;
            }

            let mut possible_paths_pheromone : f32 = 0.0;
            // Get total pheromone in possible paths
            for possible_path in &possible_paths {
                possible_paths_pheromone += possible_path.pheromone;
            }

            // Asign each path a probability to choose
            let mut probabilities : Vec<ProbPath> = Vec::new();
            for possible_path in &possible_paths {
                let probability_path = ProbPath(possible_path.pheromone / possible_paths_pheromone, &**possible_path, possible_path.get_to_city());
                probabilities.push(probability_path);
            }

            let mut rng = rand::thread_rng();
            let rand_number : f32 = rng.gen();

            // Select a path with the given probability.
            for probability in probabilities.iter() {
                if probability.0 <= rand_number {
                    let choosed_path = probability.1;
                    let c = choosed_path as *mut path::Path;
                    self.tour.push (c);
                    let index = cities_to_find.iter().position(|x| *x == unsafe { (*choosed_path).get_to_city().clone() } ).unwrap();
                    actual = unsafe { (*choosed_path).get_to_city() };
                    cities_to_find.remove(index);
                    break;
                }
            }
        }
    }

    /**
    * Return a clone of every path in tour.
    **/
    pub fn get_tour (&self) -> Vec::<path::Path> {
        let mut tour : Vec::<path::Path> = Vec::new();

        for path in &self.tour {
            unsafe { tour.push((**path).clone()) };
        }
        return tour;
    }

    /**
    * Remove a path from possible_paths.
    *
    * possible_paths : vector of possible paths.
    * possible_path : path to remove.
    */
    fn remove_path(&mut self, possible_paths : &mut Vec::<&path::Path>, possible_path : &path::Path) {
        let index = possible_paths.iter().position(|x| **x == *possible_path).unwrap();
        possible_paths.remove(index);
    }

    /**
    * Return the cost (distance) of tour.
    */
    pub fn objective_function (&self) -> f32 {
        let mut cost : f32 = 0.0;
        for path in self.tour.clone() {
            let acc = unsafe { (*path).clone() };
            cost += acc.euclidean_distance();
        }
        return cost;
    }

}

impl fmt::Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Ant {} : {:#?}]", unsafe { (*self.nest).name.clone() }, self.tour)
    }
}

impl PartialEq for Ant {
    fn eq(&self, other : &Self) -> bool {
        if unsafe { *self.nest != *other.nest } {
            return false;
        }
        if self.tour.len() != other.tour.len() {
            return false;
        }
        for i in 0..(self.tour.len()) {
            if self.tour[i] != other.tour[i] {
                return false;
            }
        }
        return true;
    }
}
