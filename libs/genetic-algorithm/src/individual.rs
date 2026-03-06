use crate::chromosome::Chromosome;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[cfg(test)]
pub use tests::TestIndividual;

#[cfg(test)]
pub mod tests {
    use super::Individual;
    use crate::chromosome::Chromosome;

    #[derive(Clone, Debug)]
    pub enum TestIndividual {
        /// For tests that require a chromosome
        WithChromosome { chromosome: Chromosome },

        /// For tests that don't require a chromosome
        WithFitness { fitness: f32 },
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,
                Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => chromosome.iter().sum(),
                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    // Implement without strict derive
    impl PartialEq for TestIndividual {
        fn eq(&self, other: &Self) -> bool {
            self.fitness() == other.fitness()
        }
    }

    impl PartialOrd for TestIndividual {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.fitness().partial_cmp(&other.fitness())
        }
    }
}
