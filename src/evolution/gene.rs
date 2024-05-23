use nannou::math::num_traits::{PrimInt, ToPrimitive};
use rand::Rng;

use crate::util::{rnd_exp, Create};

/// A CostFunction determines the cost of a Type
pub type CostFunction<T> = fn(&T) -> f32;

/// A Genome is a set of heritable pieces of information T
/// It is also able to rate each piece of information according to a CostFunction
pub type Genome<T> = Vec<T>;

pub trait Genetic<T> {
    fn rate_fitness(&self, cost_function: CostFunction<T>) -> f32;
    fn mutate_at(&mut self, at: usize);
    fn combine<S: PrimInt>(&mut self, fathers: &Vec<Self>, indices: &Vec<S>)
    where Self: Sized;
}

/// A Quadrant defines a specific subset of the Genome
pub enum Quadrant {
    TopTriangularQuadrant,
    BottomTriangularQuadrant,
    RightTriangularQuadrant,
    LeftTriangularQuadrant,
}

impl<T> Genetic<T> for Genome<T>
where
    T: Create + Clone + Copy,
{
    /// Determines the fitness of the genome
    fn rate_fitness(&self, cost_function: CostFunction<T>) -> f32 {
        // let norm = 1.0 / self.data.len() as f32;
        let sum: f32 = self.iter().map(|i| cost_function(i)).sum();
        sum
    }

    /// Replaces a PoI by another random PoI
    fn mutate_at(&mut self, at: usize) {
        self[at] = T::create();
    }

    /// Replaces subsets of the genome with subsets in "fathers"
    fn combine<S: PrimInt>(&mut self, fathers: &Vec<Self>, indices: &Vec<S>) {
        assert!(self.len() == indices.len());

        for (at, from) in indices.iter().enumerate() {
            let from = from.to_usize().unwrap();
            if from >= fathers.len() {
                continue;
            }
            self[at] = fathers[from][at];
        }
    }
}

pub trait Population<T: Create + Clone + Copy> {
    fn new_genome(size: usize) -> Genome<T>;

    fn extract_genomes(&mut self) -> Vec<&mut Genome<T>>;
}

/// Structs implementing this can evolve a population of up to <S_max> genomes T
pub trait Evolve<T, S>
where
    T: Create + Clone + Copy,
    S: PrimInt,
{
    /// Orders the population descendingly by fitness
    fn weight(population: &mut Vec<&mut Genome<T>>, cost_function: CostFunction<T>) {
        population.sort_unstable_by_key(|p| p.rate_fitness(cost_function) as i32);
    }
    /// Randomly chooses a Vec of fathers according to their fitness
    fn get_fathers(
        population: &Vec<&mut Genome<T>>,
        rho: usize,
        diversity: usize,
    ) -> Vec<Genome<T>> {
        let mut fathers = Vec::with_capacity(rho);
        while fathers.len() < rho {
            let index: usize = rnd_exp(diversity);
            fathers.push(population[index % population.len()].clone());
        }
        fathers
    }

    /// Performs a mapping m: PoI -> {mother; fathers}
    fn get_indices(genome_size: usize, fathers: &Vec<Genome<T>>) -> Vec<S> {
        assert!(fathers.len() < S::max_value().to_usize().unwrap_or(usize::max_value()));

        // mapper function which maps a genome index to a father
        let map = |_: usize| {
            let result = S::from(rnd_exp(fathers.len() / 2) % (fathers.len() + 1));
            match result {
                Some(x) => x,
                None => panic!("Failed to convert usize into this PrimNum type"),
            }
        };
        let size = genome_size.to_usize();
        let size = match size {
            Some(x) => x,
            None => panic!("Failed to convert genome_size into usize"),
        };
        // figure out the intervals at which genetic information will be copied
        let mut indices = Vec::with_capacity(size);
        for i in 0..size {
            indices.push(map(i));
        }
        indices
    }

    /// Adds <~expected> Mutations to a Genome
    fn mutate(t: &mut Genome<T>, expected: usize) {
        let mut rng = rand::thread_rng();
        let mutation_amount = rnd_exp(expected);
        for _ in 0..mutation_amount {
            let at = rng.gen_range(0..t.len());
            t.mutate_at(at);
        }
    }

    fn evolve(mut population: Vec<&mut Genome<T>>, cost_function: CostFunction<T>) {
        let size = population.len();

        Self::weight(&mut population, cost_function);

        let weights = population.iter().map(|x| x.rate_fitness(cost_function));

        for (i, w) in weights.enumerate() {
            if i > 5 {
                break;
            }
            println!("place number {} has cost {}", i, w);
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
