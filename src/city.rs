#[derive(Debug)]
pub struct City {
    #[allow(dead_code)]
    id: u32,
    name: String,
    x_axis: f32,
    y_axis: f32,
}

impl City {

    pub fn new (id: u32, name: String, x_axis: f32, y_axis:f32) -> City {
        City { id, name, x_axis, y_axis}
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id:u32) {
        self.id = id;
    }
}
