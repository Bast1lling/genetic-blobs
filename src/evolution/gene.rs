use nannou::math::num_traits::{PrimInt, ToPrimitive};
use rand::Rng;

use crate::util::{rnd_exp, Create};

/// A CostFunction determines the cost of an information T
pub type CostFunction<T> = fn(&T) -> f32;

/// A Genome is a set of heritable pieces of information T
pub type Genome<T> = Vec<T>;

/// The trait which a Genome needs to fulfill
pub trait Genetic<T> {
    /// Randomly replaces a single information inside a Genome
    fn mutate_at(&mut self, at: usize);

    /// Combines parts of yourself with other Genomes according to the indices vector
    fn combine<S: PrimInt>(&mut self, fathers: &Vec<Self>, indices: &Vec<S>)
    where
        Self: Sized;
}

/// A type which carries a Genome<T> is called a Creature
pub trait Creature<T: Create + Clone + Copy> {
    /// A Creature can provide access to its Genome through this interface method
    fn extract_genome(&mut self) -> &mut Genome<T>;
}

/// rates similarity in percent
pub trait Compare {
    fn compare(&self, to: &Self) -> f32;
}

/// Concrete implementation of the Genetic trait
impl<T: Create + Clone + Copy> Genetic<T> for Genome<T>
{
    /// Requires the information to implement Create
    fn mutate_at(&mut self, at: usize) {
        self[at] = T::create();
    }

    /// Assumes that each number in indices is pointing to a father
    /// if n >= fathers.len, the mother information is used
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

impl<T> Create for Genome<T>
where
    T: Create + Clone + Copy,
{
    type Params = usize;

    fn create() -> Self {
        todo!()
    }

    fn create_like(params: Option<Self::Params>) -> Self {
        let size = params.unwrap();
        let mut genome = Vec::with_capacity(size);
        for _ in 0..size {
            genome.push(T::create());
        }
        genome
    }
}

impl<T> Compare for Genome<T>
where T: Compare
{
    fn compare(&self, to: &Self) -> f32 {
        assert!(self.len() == to.len(), "Can not compare genomes of different sizes");
        let mut similarity = 0.0;
        let norm = self.len() as f32;
        for (a,b) in self.iter().zip(to.iter()) {
            similarity += a.compare(b);
        }
        similarity / norm
    }
}

/// Definition of a genetic algorithm operating on a population
/// T: The information type of a Genome
/// R: A population of Creatures
/// S: A number defining the maximum size of the population
pub trait Evolve<T, R, S>
where
    T: Create + Clone + Copy,
    R: Creature<T>,
    S: PrimInt,
{
    /// Puts the genome with lowest cost first
    fn weight(genome_pool: &mut Vec<&mut Genome<T>>, rate_fitness: CostFunction<Genome<T>>) {
        genome_pool.sort_unstable_by_key(|p| rate_fitness(p) as i32);
    }

    /// Randomly chooses a pool of fathers from the population
    /// Todo: remove cloning for speed up
    fn get_fathers(
        genome_pool: &Vec<&mut Genome<T>>,
        rho: usize,
        diversity: usize,
    ) -> Vec<Genome<T>> {
        let mut fathers = Vec::with_capacity(rho);
        while fathers.len() < rho {
            let index: usize = rnd_exp(diversity);
            fathers.push(genome_pool[index % genome_pool.len()].clone());
        }
        fathers
    }

    /// Performs a mapping for (every t in Genome<T>) to its (parent in {mother; fathers})
    fn get_indices(genome_size: usize, fathers: &Vec<Genome<T>>) -> Vec<S> {
        assert!(fathers.len() < S::max_value().to_usize().unwrap_or(usize::max_value()));

        // mapper function which maps a genome index to a father
        let map = |_: usize| {
            let result = S::from(rnd_exp((fathers.len() / 2).clamp(1, fathers.len())) % (fathers.len() + 1));
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

    /// One iteration of a genetic algorithm
    /// It manipulates the genomes of a population to form a new generation
    fn evolve(population: &mut Vec<R>, rate_fitness: CostFunction<Genome<T>>) {
        let mut genome_pool: Vec<&mut Genome<T>> = Vec::with_capacity(population.len());
        for creature in population.iter_mut() {
            genome_pool.push(creature.extract_genome());
        }
        let size = genome_pool.len();
        Self::weight(&mut genome_pool, rate_fitness);

        let mut index = size;

        while index > 0 {
            let fathers = Self::get_fathers(&genome_pool, 1, size / 2);
            let mother = &mut genome_pool[index - 1];
            let genome_size = mother.len();
            let indices = Self::get_indices(genome_size, &fathers);
            mother.combine(&fathers, &indices);
            let expected = (2).clamp(1, genome_size - 1);
            Self::mutate(mother, expected);
            index -= 1;
        }
    }
}
