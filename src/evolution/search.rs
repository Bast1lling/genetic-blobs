use rand::Rng;
use super::blob::{Blob, Gene};
use crate::util::util::rnd_exp;

pub trait Evolve<T> where T: Gene + Clone {
    fn weight(population: &mut Vec<T>);
    fn get_fathers(population: &Vec<T>, rho: usize, diversity: usize) -> Vec<T>;
    fn reproduce(mother: &T, fathers: &Vec<T>) -> T;
    fn mutate(t: &mut T, expected: usize);
    fn get(&mut self) -> &mut Vec<T>;
    fn set(&mut self, new_population: Vec<T>);

    fn evolve(&mut self) {
        let population = self.get();
        let size = population.len();
        let mut children = Vec::with_capacity(size);
    
        Self::weight(population);
    
        let weights = population.iter().map(|x| x.rate_fitness());
    
        for (i,w) in weights.enumerate() {
            if i > 10 {
                break;
            }
            println!("place number {} has score {}", i , w);
        }
    
        for mother in population.iter() {
            let fathers = Self::get_fathers(population, 4, size/2);
            let mut child = Self::reproduce(mother, &fathers);
            let genome_size = child.length();
            let expected = (genome_size / 20).clamp(1, genome_size - 1);
            Self::mutate(&mut child, expected);
            children.push(child);
        }
    
        self.set(children);
    }
}

#[derive(Clone)]
pub struct SimpleBlobPopulation {
    pub population: Vec<Blob>,
}

impl SimpleBlobPopulation {
    pub fn new(population: Vec<Blob>) -> Self {
        Self { population }
    }
}

impl Evolve<Blob> for SimpleBlobPopulation {
    fn weight(population: &mut Vec<Blob>) {
        population.sort_unstable_by_key(|p| -p.rate_fitness() as i32);
    }

    fn get_fathers(population: &Vec<Blob>, rho: usize, diversity: usize) -> Vec<Blob> {
        let mut fathers = Vec::with_capacity(rho);
        while fathers.len() < rho {
            let index: usize = rnd_exp(diversity);
            fathers.push(population[index % population.len()].clone());
        }
        fathers
    }

    fn reproduce(mother: &Blob, fathers: &Vec<Blob>) -> Blob {
        assert!(fathers.len() < 256);
    
        // mapper function which maps a genome index to a father
        let map = |_: usize| {
            (rnd_exp(fathers.len()/2) % (fathers.len() + 1)) as u8
        };
    
        // figure out the intervals at which genetic information will be copied
        let mut indices = Vec::with_capacity(mother.length());
        for i in 0..mother.length() {
            indices.push(map(i));
        }
        Blob::combine(mother, fathers, &indices)
    }

    fn mutate(t: &mut Blob, expected: usize) {
        let mut rng = rand::thread_rng();
        let mutation_amount = rnd_exp(expected);
        for _ in 0..mutation_amount {
            let at = rng.gen_range(0..t.length());
            t.mutate(at);
        }
    }
    
    fn get(&mut self) -> &mut Vec<Blob> {
        &mut self.population
    }
    
    fn set(&mut self, new_population: Vec<Blob>) {
        self.population = new_population;
    }
}

