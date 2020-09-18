use crate::city;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    #[allow(dead_code)]
    pub from_city : *const city::City,
    pub to_city : *const city::City,
    pub pheromone : f32,
}

impl Path {
    pub fn new(from_city : &city::City, to_city : &city::City) -> Path {
        Path { from_city, to_city, pheromone: 0.50}
    }

    pub fn get_from_city (&self) -> &city::City {
        unsafe { &*self.from_city }
    }

    pub fn get_to_city (&self) -> &city::City {
        unsafe { &*self.to_city }
    }

}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}]", unsafe { (*self.from_city).name.clone() },  unsafe { (*self.to_city).name.clone() })
    }
}

impl PartialEq for Path {
    fn eq(&self, other : &Self) -> bool {
        self.from_city == other.from_city && self.to_city == other.to_city && self.pheromone == other.pheromone
    }
}
