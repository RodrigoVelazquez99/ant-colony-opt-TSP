use crate::city;
use std::fmt;

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

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}]", self.from_city.name, self.to_city.name)
    }
}
