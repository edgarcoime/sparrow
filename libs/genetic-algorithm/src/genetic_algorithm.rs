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

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::crossover::UniformCrossover;
    use crate::individual::{Individual, TestIndividual};
    use crate::mutation::GaussianMutation;
    use crate::selection::RouletteWheelSelection;
    use crate::GeneticAlgorithm;

    #[test]
    fn genetic_algorithm() {
        // Helper to create individuals
        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population: Vec<TestIndividual> = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population: Vec<TestIndividual> = vec![
            individual(&[0.4476949, 2.0648358, 4.3058133]),
            individual(&[1.2126867, 1.5538777, 2.886911]),
            individual(&[1.0617678, 2.265739, 4.428764]),
            individual(&[0.95909685, 2.4618788, 4.024733]),
        ];

        assert_eq!(population, expected_population);
    }
}
