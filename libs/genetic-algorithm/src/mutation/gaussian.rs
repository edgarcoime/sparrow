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
    use super::*;

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}
