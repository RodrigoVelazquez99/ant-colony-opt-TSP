use crate::city;
use crate::path;
use std::fmt;
use rand::Rng;

#[derive(Debug)]
pub struct Ant {
    #[allow(dead_code)]
    tour : Vec<*const path::Path>,
    nest : *const city::City
}

// Pair for the probability of choose the path
struct ProbPath (f32, *const path::Path, *const city::City);

impl Ant {
    pub fn new (nest : &city::City) -> Ant {
        Ant {nest, tour : Vec::new()}
    }

    // Build a tour from all paths in graph
    pub fn get_tour (&mut self, graph : &Vec::<path::Path>, world : &Vec::<city::City>) {

        let mut actual = unsafe { &*self.nest.clone() };
        let mut cities_to_find : Vec<&city::City> = world.into_iter().filter(|x| **x != *actual ).collect::<Vec<&city::City>>().clone();
        while ! cities_to_find.is_empty() {

            let mut possible_paths : Vec::<&path::Path> = graph.into_iter().filter(|x| *x.get_from_city() == *actual).collect::<Vec<&path::Path>>().clone();

            let mut t = self.tour.clone();
            let searched_cities = t.into_iter().map(|x| unsafe { (*x).get_from_city() }).collect::<Vec<&city::City>>();
            for path in possible_paths.clone() {
                let to_city = unsafe { &*path.get_to_city() };
                let is_in_cities = searched_cities.iter().position(|x| **x == unsafe { (*to_city).clone() });
                match is_in_cities {
                    Some(i) => self.remove_path(&mut possible_paths, &path),
                    None => continue,
                }
            }

            println!("ciudad actual {} \n", unsafe { (*actual).clone()} );
            println!("caminos posibles : {}\n", possible_paths.len() );
            let mut possible_paths_pheromone : f32 = 0.0;

            // There is only a path avaible
            if possible_paths.len() == 1 {
                let final_path = possible_paths[0];
                self.tour.push(final_path);
                actual = unsafe { &(final_path).get_to_city() };
                //println!("ciudad final {}", unsafe { (*possible_paths[0]).get_to_city().clone() });
                cities_to_find.clear();
                // Path between nest and final city
                let return_path = graph.iter().find(|x| unsafe { (*x).get_to_city().clone() == (*self.nest).clone() && (*x).get_from_city() == actual}).unwrap();
                self.tour.push(return_path);
                println!("\n\n EL TOUR ES: \n\n", );
                for p in self.tour.clone() {
                    println!("{}", unsafe { (*p).clone() });
                }
                break;
            }

            // Get total probability from possible paths
            for possible_path in &possible_paths {
                possible_paths_pheromone += possible_path.pheromone;
            }
            // Asign each path a probability to choose
            let mut probabilities : Vec<ProbPath> = Vec::new();
            for possible_path in &possible_paths {
                let probability_path = ProbPath(possible_path.pheromone / possible_paths_pheromone, &**possible_path, possible_path.get_to_city());
                probabilities.push(probability_path);
            }

            // let rand_number = rand::thread_rng().gen_range(1,101);
            let mut rng = rand::thread_rng();
            let rand_number : f32 = rng.gen();


            for probability in probabilities.iter() {
                if probability.0 <= rand_number {
                    let choosed_path = probability.1;
                    self.tour.push (choosed_path);
                    println!("{}", unsafe { (*choosed_path).clone() });
                    println!("ciudad a la que se dirige {}", unsafe { (*choosed_path).get_to_city() });
                    let index = cities_to_find.iter().position(|x| *x == unsafe { (*choosed_path).get_to_city() } ).unwrap();
                    actual = unsafe { (*choosed_path).get_to_city() };
                    println!("nueva ciudad actual {}", actual);
                    cities_to_find.remove(index);
                    let index_path = probabilities.iter().position(|x| unsafe { (*x.1).clone() } == unsafe { (*choosed_path).clone() }).unwrap();
                    println!("index_path {}", index_path);
                    println!("{}",unsafe { (*probabilities.remove(index_path).1).clone()});
                    break;
                }
            }
            println!("\n ------ Iteracion \n");
        }

    }

    fn remove_path(&mut self, possible_paths : &mut Vec::<&path::Path>, possible_path : &path::Path) {
        let index = possible_paths.iter().position(|x| *x == possible_path).unwrap();
        possible_paths.remove(index);
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
