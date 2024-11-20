use crate::*;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    //WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    //fn create(chromosome: Chromosome) -> Self {
    //    Self::WithChromosome { chromosome }
    //}
    //

    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual");
        //match self {
        //    Self::WithChromosome { chromosome } => chromosome,
        //    Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
        //}
    }

    fn fitness(&self) -> f32 {
        match self {
            //Self::WithChromosome { chromosome } => chromosome.iter().sum(),
            Self::WithFitness { fitness } => *fitness,
        }
    }
}
