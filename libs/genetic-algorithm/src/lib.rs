mod chromosome;
mod errors;
mod genetic_algorithm;
mod individual;

pub mod crossover;
pub mod mutation;
pub mod selection;

pub use chromosome::Chromosome;
pub use genetic_algorithm::GeneticAlgorithm;
pub use individual::Individual;
