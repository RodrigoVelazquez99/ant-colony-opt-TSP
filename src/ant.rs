use crate::city;
use crate::path;

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
