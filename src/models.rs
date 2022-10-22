use nannou::prelude::*;

pub struct Model {
    pub window: WindowId
}

impl Model {
    pub fn new(w: WindowId) -> Model {
        Model { window: w }
    }
}