use crate::*;

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self::random_with_sizes(rng, 40, 60)
    }

    pub fn random_with_sizes(rng: &mut dyn RngCore, animal_count: usize, food_count: usize) -> Self {
        let animals = (0..animal_count).map(|_| Animal::random(rng)).collect();
        let foods = (0..food_count).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
