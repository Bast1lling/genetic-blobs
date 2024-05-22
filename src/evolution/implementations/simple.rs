use nannou::glam::Vec2;
use rand::Rng;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
    },
    util::{rnd_exp, Create},
    Nannou,
};

fn rate(color: &RGB) -> f32 {
    // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
    let treshold: u8 = 15;
    if color.r < treshold && color.g < treshold && color.b < treshold {
        1.0
    } else {
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct SimpleBlobPopulation {
    pub blobs: Vec<Blob>,
}

impl SimpleBlobPopulation {
    pub fn new(
        nannou_positions: Vec<Vec2>,
        nannou_size: f32,
        population_size: u16,
        genome_size: usize,
    ) -> Self {
        let population_size = population_size as usize;
        let mut blobs = Vec::with_capacity(population_size);
        //create a random population
        for i in 0..population_size {
            let genome = Self::new_genome(genome_size);
            let nannou_position = nannou_positions[i];
            let blob = Blob::new(genome, nannou_size, nannou_position);
            blobs.push(blob);
        }

        SimpleBlobPopulation { blobs }
    }

    fn new_genome(size: usize) -> Genome<RGB> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(RGB::new());
        }
        Genome { data }
    }

    fn extract_genomes(&mut self) -> Vec<&mut Genome<RGB>> {
        let mut genomes: Vec<&mut Genome<RGB>> = Vec::with_capacity(self.blobs.len());
        for blob in self.blobs.iter_mut() {
            genomes.push(&mut blob.genome);
        }
        genomes
    }
}

impl Nannou for SimpleBlobPopulation {
    fn draw(&self, draw: &nannou::prelude::Draw, model: &crate::Model) {
        for blob in &self.blobs {
            blob.draw(&draw, &model);
        }
    }

    fn update(&mut self) {
        let genome_references = self.extract_genomes();
        Self::evolve(genome_references, rate);
    }
}

impl Evolve<RGB, u16> for SimpleBlobPopulation {
    fn weight(population: &mut Vec<&mut Genome<RGB>>) {
        population.sort_unstable_by_key(|p| -p.rate_fitness(rate) as i32);
    }

    fn get_fathers(
        population: &Vec<&mut Genome<RGB>>,
        rho: usize,
        diversity: usize,
    ) -> Vec<Genome<RGB>> {
        let mut fathers = Vec::with_capacity(rho);
        while fathers.len() < rho {
            let index: usize = rnd_exp(diversity);
            fathers.push(population[index % population.len()].clone());
        }
        fathers
    }

    fn get_indices(size: usize, fathers: &Vec<Genome<RGB>>) -> Vec<u16> {
        assert!(fathers.len() < u16::max_value() as usize);

        // mapper function which maps a genome index to a father
        let map = |_: usize| (rnd_exp(fathers.len() / 2) % (fathers.len() + 1)) as u16;

        // figure out the intervals at which genetic information will be copied
        let mut indices = Vec::with_capacity(size);
        for i in 0..size {
            indices.push(map(i));
        }
        indices
    }

    fn mutate(t: &mut Genome<RGB>, expected: usize) {
        let mut rng = rand::thread_rng();
        let mutation_amount = rnd_exp(expected);
        for _ in 0..mutation_amount {
            let at = rng.gen_range(0..t.len());
            t.mutate_at(at);
        }
    }
}
