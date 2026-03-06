use rand::RngCore;

use crate::selection::SelectionMethod;
use crate::crossover::CrossoverMethod;
use crate::individual::Individual;

pub struct GeneticAlgorithm<S, X> {
    selection_method: S,
    crossover_method: X,
}

impl<S, X> GeneticAlgorithm<S, X>
where
    S: SelectionMethod,
    X: CrossoverMethod,
{
    pub fn new(selection_method: S, crossover_method: X) -> Self {
        Self {
            selection_method,
            crossover_method,
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                // SELECTION
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // CROSSOVER
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // TODO mutation
                todo!()
            })
            .collect()
    }
}
