// Cannot implement Individual w/o knowing implementation of chromosome
// So create seperate struct instead

use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        todo!()
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        todo!()
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome()
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}