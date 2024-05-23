use std::ops::Neg;

use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
    },
    util::Create,
};

use super::gene::CostFunction;

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

pub trait Population<T: Create + Clone + Copy> {
    fn new_genome(cost_function: CostFunction<T>, size: usize) -> Genome<T>;
    
    fn extract_genomes(&mut self) -> Vec<&mut Genome<T>>;
}

/// Represents a collection of Blobs which are able to evolve
pub type SimpleBlobPopulation = Vec<Blob>;

impl Evolve<RGB, u16> for SimpleBlobPopulation {}

impl Population<RGB> for SimpleBlobPopulation {
    fn new_genome(cost_function: fn(&RGB) -> f32, size: usize) -> Genome<RGB> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(RGB::create());
        }
        Genome { data, cost_function }
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
    type Params = (Vec<Vec2>, f32, u16, usize, CostFunction<RGB>);
    
    fn create_like(params: Option<Self::Params>) -> Self {
        let (nannou_positions,
            nannou_size,
            population_size,
            genome_size,
            cost_function
        ) = params.unwrap();
        let population_size = population_size as usize;
        let mut blobs = Vec::with_capacity(population_size);
        //create a random population
        for i in 0..population_size {
            let genome = Self::new_genome(cost_function, genome_size);
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
