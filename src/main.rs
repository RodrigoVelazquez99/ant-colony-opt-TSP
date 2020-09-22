use rand::Rng;
use std::time::{Instant};
mod city;
mod path;
mod ant;

fn main() {
    let world = init_data_cities();
    let graph = init_graph(&world);
    aco(world, graph, 0.60, 0.20);
}

// Ant Colony Optimization algorithm
fn aco (world : Vec<city::City>, mut graph : Vec<path::Path>, p : f32, q : f32) {
    let mut ants : Vec<ant::Ant> = Vec::new();
    let mut best_tour : Vec<path::Path> = Vec::new();
    let mut best_objective = 1000000000000000.0;
    let time = Instant::now();
    for _ in 1..=5 {
        for _n in 1..=50 {
            // Take random nest
            let mut rng = rand::thread_rng();
            let rand_number = rng.gen_range(0, 37);
            let nest = &world[rand_number];
            ants.push(ant::Ant::new(nest));
        }
        // Each ant gets a tour from the graph
        for ant in &mut ants {
            ant.get_tour(&graph, &world);
        }
        // Update paths
        update_pheromone(&mut graph, &ants, p, q);

        // Get the best tour in current generation
        for ant in &mut ants {
            let current_objective = ant.objective_function();
            if current_objective <= best_objective {
                best_objective = current_objective;
                best_tour = ant.get_t();
            }
        }
    }
    println!("<<<<<<<<<<<<<<<<<<<<< MEJOR TOUR >>>>>>>>>>>>>>>>>>>>>");
    for s  in best_tour.clone() {
        println!("{}", s);
    }
    println!("Costo : {}", best_objective);
    println!("MEJOR TOUR >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("Tiempo {:#?}", time.elapsed() );
}

// Evaporate all pheromone in graphs and update paths in each ant tour
fn update_pheromone(graph : &mut Vec<path::Path>, ants : &Vec<ant::Ant>, p : f32, q : f32) {
    // Evaporate all paths
    for path in graph.into_iter() {
        let new_pheromone = (1.00 - p) * path.pheromone;
        path.set_pheromone ( new_pheromone);
    }

    // Update paths that were visited
    for ant in ants {
        let objective_function = ant.objective_function();
        for path in ant.tour.iter() {
            unsafe {
                let pa = graph.into_iter().find(|x| **x == **path).unwrap();
                pa.set_pheromone(pa.pheromone + q / objective_function);
            }
        }
    }

    for p in graph.into_iter() {
        println!("{}", p);
    }

}

// Create init cities
fn init_data_cities () -> Vec::<city::City> {
    let c = city::City::new (1, String::from("Ciudad 1"), 11003.611100, 42102.500000);
    let c1 = city::City::new (2, String::from("Ciudad 2"), 11108.611100, 42373.888900);
    let c2 = city::City::new (3, String::from("Ciudad 3"), 11133.333300, 42885.833300);
    let c3 = city::City::new (4, String::from("Ciudad 4"), 11155.833300, 42712.500000);
    let c4 = city::City::new (5, String::from("Ciudad 5"), 11183.333300, 42933.333300);
    let c5 = city::City::new (6, String::from("Ciudad 6"), 11297.500000, 42853.333300);
    let c6 = city::City::new (7, String::from("Ciudad 7"), 11310.277800, 42929.444400);
    let c7 = city::City::new (8, String::from("Ciudad 8"), 11416.666700, 42983.333300);
    let c8 = city::City::new (9, String::from("Ciudad 9"), 11423.888900, 43000.277800);
    let c9 = city::City::new (10, String::from("Ciudad 10"), 11438.333300, 42057.222200);

    let c10 = city::City::new (11, String::from("Ciudad 11"), 11461.111100, 43252.777800);
    let c11 = city::City::new (12, String::from("Ciudad 12"), 11485.555600, 43187.222200);
    let c12 = city::City::new (13, String::from("Ciudad 13"), 11503.055600, 42855.277800);
    let c13 = city::City::new (14, String::from("Ciudad 14"), 11511.388900, 42106.388900);
    let c14 = city::City::new (15, String::from("Ciudad 15"), 11522.222200, 42841.944400);
    let c15 = city::City::new (16, String::from("Ciudad 16"), 11569.444400, 43136.666700);
    let c16 = city::City::new (17, String::from("Ciudad 17"), 11583.333300, 43150.000000);
    let c17 = city::City::new (18, String::from("Ciudad 18"), 11595.000000, 43148.055600);
    let c18 = city::City::new (19, String::from("Ciudad 19"), 11600.000000, 43150.000000);
    let c19 = city::City::new (20, String::from("Ciudad 20"), 11690.555600, 42686.666700);

    let c20 = city::City::new (21, String::from("Ciudad 21"), 11715.833300, 41836.111100);
    let c21 = city::City::new (22, String::from("Ciudad 22"), 11751.111100, 42814.444400);
    let c22 = city::City::new (23, String::from("Ciudad 23"), 11770.277800, 42651.944400);
    let c23 = city::City::new (24, String::from("Ciudad 24"), 11785.277800, 42884.444400);
    let c24 = city::City::new (25, String::from("Ciudad 25"), 11822.777800, 42673.611100);
    let c25 = city::City::new (26, String::from("Ciudad 26"), 11846.944400, 42660.555600);
    let c26 = city::City::new (27, String::from("Ciudad 27"), 11963.055600, 43290.555600);
    let c27 = city::City::new (28, String::from("Ciudad 28"), 11973.055600, 43026.111100);
    let c28 = city::City::new (29, String::from("Ciudad 29"), 12058.333300, 42195.555600);
    let c29 = city::City::new (30, String::from("Ciudad 30"), 12149.444400, 42477.500000);

    let c30 = city::City::new (31, String::from("Ciudad 31"), 12286.944400, 43355.555600);
    let c31 = city::City::new (32, String::from("Ciudad 32"), 12300.000000, 42433.333300);
    let c32 = city::City::new (33, String::from("Ciudad 33"), 12355.833300, 43156.388900);
    let c33 = city::City::new (34, String::from("Ciudad 34"), 12363.333300, 43189.166700);
    let c34 = city::City::new (35, String::from("Ciudad 35"), 12372.777800, 42711.388900);
    let c35 = city::City::new (36, String::from("Ciudad 36"), 12386.666700, 43334.722200);
    let c36 = city::City::new (37, String::from("Ciudad 37"), 12421.666700, 42895.555600);
    let c37 = city::City::new (38, String::from("Ciudad 38"), 12645.000000, 42973.333300);

    vec![c, c1, c2, c3, c4, c5, c6, c7, c8, c9,
     c10, c11, c12, c13, c14, c15, c16, c17, c18,
      c19, c20, c21, c22, c23, c24, c25, c26, c27, c28,
       c29, c30, c31, c32, c33, c34, c35, c36, c37]
}

// Build the init graph with all posible paths.
fn init_graph (world : &Vec<city::City>) -> Vec<path::Path> {
    let mut graph : Vec<path::Path> = Vec::new();

    for city in world.iter() {
        for city_ in world.iter() {
            if city != city_ {
                let new_path = path::Path::new(city, city_);
                graph.push(new_path);
            }
        }
    }
    return graph;
}
