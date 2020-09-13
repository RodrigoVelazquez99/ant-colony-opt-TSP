use crate::city;
use crate::path;
use std::fmt;


#[derive(Debug)]
pub struct Ant {
    #[allow(dead_code)]
    tour : Vec<path::Path>,
    actual_position : city::City,
    nest : city::City,
    end : city::City
}

impl Ant {

}

impl fmt::Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Ant : {}]", self.actual_position.name)
    }
}
