use crate::city;

#[derive(Debug)]
pub struct Path {
    #[allow(dead_code)]
    from_city : city::City,
    to_city : city::City,
    pheromone : f32,
}

impl Path {
    pub fn new(from_city : city::City, to_city : city::City) -> Path {
        Path { from_city, to_city, pheromone: 0.50}
    }
}
