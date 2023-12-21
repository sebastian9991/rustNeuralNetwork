use rand::{seq::SliceRandom, RngCore};
use std::ops::Index;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}
pub struct RouletteWheelSelection;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual;
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population")
    }
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<T>(&self, rng: &mut dyn RngCore, population: &[T]) -> Vec<T>
    where
        T: Individual,
    {
        (0..population.len())
            .map(|_| {
                //Parent choice
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);
                todo!();
            })
            .collect()
    }
}

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

//Implemntations of std library functions for Chromosome:
//
//
//
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

#![feature(type_alias_impl_trait)]
impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod selection {

        use super::*;

        #[test]
        fn test() {
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;
            use std::{collections::BTreeMap, iter::FromIterator};
            let method = RouletteWheelSelection::new();
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let population = vec![
                TestIndividual::new(2.0),
                TestIndividual::new(1.0),
                TestIndividual::new(4.0),
                TestIndividual::new(3.0),
            ];

            let mut actual_histogram = BTreeMap::new();

            for _ in 0..1000 {
                let fitness = method.select(&mut rng, &population).fitness() as i32;

                *actual_histogram.entry(fitness).or_insert(0) += 1;
            }

            let actual_histogram: BTreeMap<i32, _> = (0..1000)
                .map(|_| method.select(&mut rng, &population))
                .fold(Default::default(), |mut histogram, individual| {
                    *histogram.entry(individual.fitness() as _).or_default() += 1;
                    histogram
                });

            let expected_histogram = maplit::btreemap! {
                1 => 101,
                2 => 192,
                3 => 293,
                4 => 414,
            };

            assert_eq!(actual_histogram, expected_histogram);
        }
    }

    mod gene {
        use super::*;

        fn chromosome() -> Chromosome {
            Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            }
        }

        mod len {
            use super::*;

            #[test]
            fn test() {
                assert_eq!(chromosome().len(), 3);
            }
        }

        mod iter {
            use super::*;

            #[test]
            fn test() {
                let chromosome = chromosome();
                let genes: Vec<_> = chromosome.iter().collect();

                assert_eq!(genes.len(), 3);
                assert_eq!(genes[0], &3.0);
                assert_eq!(genes[1], &1.0);
                assert_eq!(genes[2], &2.0);
            }
        }

        mod iter_mut {
            use super::*;

            #[test]
            fn test() {
                let mut chromosome = chromosome();

                chromosome.iter_mut().for_each(|gene| {
                    *gene *= 10.0;
                });

                let genes: Vec<_> = chromosome.iter_mut().collect();

                assert_eq!(genes.len(), 3);
                assert_eq!(genes[0], &30.0);
                assert_eq!(genes[1], &10.0);
                assert_eq!(genes[2], &20.0);
            }
        }

        mod from_iter {
            use super::*;

            #[test]
            fn test() {
                let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

                assert_eq!(chromosome[0], 3.0);
                assert_eq!(chromosome[1], 1.0);
                assert_eq!(chromosome[2], 2.0);
            }
        }
    }
}
