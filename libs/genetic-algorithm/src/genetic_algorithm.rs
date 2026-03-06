use rand::RngCore;

use crate::crossover::CrossoverMethod;
use crate::individual::Individual;
use crate::mutation::MutationMethod;
use crate::selection::SelectionMethod;

pub struct GeneticAlgorithm<S, X, M> {
    selection_method: S,
    crossover_method: X,
    mutation_method: M,
}

impl<S, X, M> GeneticAlgorithm<S, X, M>
where
    S: SelectionMethod,
    X: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: X, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
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

                // MUTATION
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect()
    }
}
