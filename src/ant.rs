use crate::city;
use crate::path;
use std::fmt;


#[derive(Debug)]
pub struct Ant {
    #[allow(dead_code)]
    tour : Vec<*const path::Path>,
    nest : *const city::City
}

impl Ant {
    pub fn new (nest : &city::City) -> Ant {
        Ant {nest, tour : Vec::new()}
    }

}

impl fmt::Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Ant {} : {:#?}]", unsafe { (*self.nest).name.clone() }, self.tour)
    }
}

impl PartialEq for Ant {
    fn eq(&self, other : &Self) -> bool {
        if ( unsafe { *self.nest != *other.nest } ) {
            return false;
        }
        if (self.tour.len() != other.tour.len()) {
            return false;
        }
        for i in 0..(self.tour.len()) {
            if (self.tour[i] != other.tour[i]) {
                return false;
            }
        }
        return true;
    }
}
