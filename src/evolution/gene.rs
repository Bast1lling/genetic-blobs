use nannou::math::num_traits::{PrimInt, ToPrimitive};
use rand::Rng;

use crate::util::{rnd_exp, Create};

pub enum Quadrant {
    TopTriangularQuadrant,
    BottomTriangularQuadrant,
    RightTriangularQuadrant,
    LeftTriangularQuadrant,
}

pub type CostFunction<T> = fn(&T) -> f32;
/// A genome is a set of heritable pieces of information (PoI)
#[derive(Debug, Clone)]
pub struct Genome<T>
where
    T: Create + Clone + Copy,
{
    pub data: Vec<T>,
    pub cost_function: CostFunction<T>,
}

impl<T> Genome<T>
where
    T: Create + Clone + Copy,
{
    /// Determines the fitness of the genome
    pub fn rate_fitness(&self) -> f32 {
        // let norm = 1.0 / self.data.len() as f32;
        let sum: f32 = self.data.iter().map(|i| (self.cost_function)(i)).sum();
        sum
    }

    /// Replaces a PoI by another random PoI
    pub fn mutate_at(&mut self, at: usize) {
        self.data[at] = T::create();
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

    pub fn get_quadrant(&self, quadrant: Quadrant) -> Vec<&T> {
        match quadrant {
            Quadrant::RightTriangularQuadrant => todo!(),
            _ => panic!("Such a quadrant does not exist!")
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
    fn weight(population: &mut Vec<&mut Genome<T>>) {
        population.sort_unstable_by_key(|p| p.rate_fitness() as i32);
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

    fn evolve(mut population: Vec<&mut Genome<T>>) {
        let size = population.len();

        Self::weight(&mut population);

        let weights = population.iter().map(|x| x.rate_fitness());

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
