pub use self::{food::*, animal::*, world::*};

mod world;
mod animal;
mod food;

pub use crate::{animal::*, food::*, world::*};

use nalgebra as na;
use rand::{Rng, RngCore};


pub struct Simulation {
    world: World,
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationConfig {
    pub animal_count: usize,
    pub food_count: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            animal_count: 40,
            food_count: 60,
        }
    }
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self::random_with_config(rng, SimulationConfig::default())
    }

    pub fn random_with_config(rng: &mut dyn RngCore, config: SimulationConfig) -> Self {
        Self {
            world: World::random_with_sizes(rng, config.animal_count, config.food_count),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
