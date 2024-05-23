use std::ops::Neg;

use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
    },
    util::Create,
};

pub fn _black_costs(color: &RGB) -> f32 {
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

impl Evolve<RGB, Blob, u16> for SimpleBlobPopulation {}

impl Create for SimpleBlobPopulation {
    type Params = (Vec<Vec2>, f32, u16, usize);

    fn create_like(params: Option<Self::Params>) -> Self {
        let (nannou_positions, nannou_size, population_size, genome_size) = params.unwrap();
        let population_size = population_size as usize;
        let mut blobs = Vec::with_capacity(population_size);
        //create a random population
        for i in 0..population_size {
            let genome = Genome::create_like(Some(genome_size));
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
