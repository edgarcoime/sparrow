use crate::prelude::*;
use crate::world::World;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    #[wasm_bindgen(js_name = withWorldSizes)]
    pub fn with_world_sizes(animal_count: usize, food_count: usize) -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random_with_config(
            &mut rng,
            sim::SimulationConfig {
                animal_count,
                food_count,
            },
        );

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step();
    }
}
