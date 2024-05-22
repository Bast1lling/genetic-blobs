use nannou::math::num_traits::PrimInt;

use crate::util::Create;

/// A genome is a set of heritable pieces of information (PoI)
#[derive(Debug, Clone)]
pub struct Genome<T>
where
    T: Create + Clone + Copy,
{
    pub data: Vec<T>,
}

impl<T> Genome<T>
where
    T: Create + Clone + Copy,
{
    /// Determines the fitness of the genome
    pub fn rate_fitness(&self, rate: fn(&T) -> f32) -> f32 {
        // let norm = 1.0 / self.data.len() as f32;
        let sum: f32 = self.data.iter().map(|i| rate(i)).sum();
        sum
    }

    /// Replaces a PoI by another random PoI
    pub fn mutate_at(&mut self, at: usize) {
        self.data[at] = T::new();
    }

    /// Returns the amount of PoIs contained by the genome
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Replaces subsets of the genome with subsets in "fathers"
    pub fn combine<S: PrimInt>(&mut self, fathers: &Vec<Self>, indices: &Vec<S>) {
        assert!(self.data.len() == indices.len());

        for (at, from) in indices.iter().enumerate() {
            let from = from.to_usize().unwrap();
            if from >= fathers.len() {
                continue;
            }
            self.data[at] = fathers[from].data[at];
        }
    }
}

/// Structs implementing this can evolve a population of up to <S_max> genomes T
pub trait Evolve<T, S>
where
    T: Create + Clone + Copy,
    S: PrimInt,
{
    /// Orders the population descendingly by fitness
    fn weight(population: &mut Vec<&mut Genome<T>>);

    /// Randomly chooses a Vec of fathers according to their fitness
    fn get_fathers(
        population: &Vec<&mut Genome<T>>,
        rho: usize,
        diversity: usize,
    ) -> Vec<Genome<T>>;

    /// Performs a mapping m: PoI -> {mother; fathers}
    fn get_indices(genome_size: usize, fathers: &Vec<Genome<T>>) -> Vec<S>;

    /// Adds <~expected> Mutations to a Genome
    fn mutate(t: &mut Genome<T>, expected: usize);

    fn evolve(mut population: Vec<&mut Genome<T>>, rate: fn(&T) -> f32) {
        let size = population.len();

        Self::weight(&mut population);

        let weights = population.iter().map(|x| x.rate_fitness(rate));

        for (i, w) in weights.enumerate() {
            if i > 5 {
                break;
            }
            println!("place number {} has score {}", i, w);
        }

        let mut index = size;

        while index > 0 {
            let fathers = Self::get_fathers(&population, 4, size / 2);
            let mother = &mut population[index - 1];
            let genome_size = mother.len();
            let indices = Self::get_indices(genome_size, &fathers);
            mother.combine(&fathers, &indices);
            let expected = (2).clamp(1, genome_size - 1);
            Self::mutate(mother, expected);
            index -= 1;
        }
    }
}
