use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
    },
    util::Create,
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
        Genome { data, rate }
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
        Self::evolve(genome_references);
    }
}

impl Evolve<RGB, u16> for SimpleBlobPopulation {}
