use crate::prelude::*;
use crate::animal::Animal;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();

        Self { animals }
    }
}
