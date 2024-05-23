use std::ops::Neg;

use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
    },
    util::Create,
};

use super::gene::Population;

pub fn black_costs(color: &RGB) -> f32 {
    // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
    let treshold: u8 = 15;
    if color.r < treshold && color.g < treshold && color.b < treshold {
        0.0
    } else {
        1.0
    }
}

pub fn red_ratio(color: &RGB) -> f32 {
    (((color.r as u16 > 4 * (color.g as u16 + color.b as u16)) as u8) as f32).neg()
}

/// Represents a collection of Blobs which are able to evolve
pub type SimpleBlobPopulation = Vec<Blob>;

impl Evolve<RGB, u16> for SimpleBlobPopulation {}

impl Population<RGB> for SimpleBlobPopulation {
    fn new_genome(size: usize) -> Genome<RGB> {
        let mut genome = Vec::with_capacity(size);
        for _ in 0..size {
            genome.push(RGB::create());
        }
        genome
    }

    fn extract_genomes(&mut self) -> Vec<&mut Genome<RGB>> {
        let mut genomes: Vec<&mut Genome<RGB>> = Vec::with_capacity(self.len());
        for blob in self.iter_mut() {
            genomes.push(&mut blob.genome);
        }
        genomes
    }
}

impl Create for SimpleBlobPopulation {
    type Params = (Vec<Vec2>, f32, u16, usize);

    fn create_like(params: Option<Self::Params>) -> Self {
        let (nannou_positions, nannou_size, population_size, genome_size) =
            params.unwrap();
        let population_size = population_size as usize;
        let mut blobs = Vec::with_capacity(population_size);
        //create a random population
        for i in 0..population_size {
            let genome = Self::new_genome(genome_size);
            let nannou_position = nannou_positions[i];
            let blob = Blob::new(genome, nannou_size, nannou_position);
            blobs.push(blob);
        }
        blobs
    }

    fn create() -> Self {
        todo!()
    }
}
