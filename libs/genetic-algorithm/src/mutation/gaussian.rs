use rand::{Rng, RngCore};

use super::MutationMethod;
use crate::*;

pub struct GaussianMutation {
    /// Probability of mutation for each gene:
    /// - 0.0 = no genes will be mutated
    /// - 1.0 = all genes will be mutated
    chance: f32,

    /// Magnitude for each gene
    /// - 0.0 = touched genes will not be modified
    /// - 3.0 = touched genes will be modified by += or -= 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    mod given_zero_chance {
        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }
    }

    mod given_fifty_fifty_chance {
        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn slightly_changes_the_original_chromosome() {
                todo!();
            }
        }
    }

    mod given_max_chance {
        mod and_zero_coefficient {
            #[test]
            fn does_not_change_the_original_chromosome() {
                todo!();
            }
        }

        mod and_nonzero_coefficient {
            #[test]
            fn entirely_changes_the_original_chromosome() {
                todo!();
            }
        }
    }
}
