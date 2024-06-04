use std::ops::Neg;

use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::{Blob, RGB},
        gene::{Evolve, Genome},
        square::{Quadrant, Square},
    },
    util::Create,
};

use super::gene::Compare;

pub fn _black_costs(color: &RGB) -> f32 {
    // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
    let treshold: u8 = 15;
    if color.r < treshold && color.g < treshold && color.b < treshold {
        0.0
    } else {
        1.0
    }
}

pub fn _red_ratio(color: &RGB) -> f32 {
    (((color.r as u16 > 4 * (color.g as u16 + color.b as u16)) as u8) as f32).neg()
}

pub fn extract_velocity(genome: &Genome<RGB>) -> Vec2 {
    let direction_weights = [
        genome.get_quadrant(Quadrant::RightTriangularQuadrant),
        genome.get_quadrant(Quadrant::TopTriangularQuadrant),
        genome.get_quadrant(Quadrant::LeftTriangularQuadrant),
        genome.get_quadrant(Quadrant::BottomTriangularQuadrant),
    ];
    let max = (direction_weights[0].len() * 255) as f32;
    let mut direction_weights: [f32; 4] = direction_weights
        .map(|list| list.iter()
                                .map(|&x| x.r as f32 - x.g as f32 - x.b as f32)
                                .sum());
    direction_weights = direction_weights.map(|x| x/max);
    let directions = [
        Vec2::new(1., 0.),
        Vec2::new(0., 1.),
        Vec2::new(-1., 0.),
        Vec2::new(0., -1.),
    ];
    let velocity: Vec2 = direction_weights.iter()
        .zip(directions.iter())
        .map(|(&weight, &dir)| dir * weight)
        .fold(Vec2::ZERO, |acc, v| acc + v);
    velocity
}

pub fn move_to(genome: &Genome<RGB>, direction: Vec2) -> f32 {
    let mut v = extract_velocity(genome);
    let mag = v.length();
    v = v.normalize();
    let angle = direction.dot(v);
    (-mag * angle).exp()
}

pub fn compare_to(genome: &Genome<RGB>, reference: &Genome<RGB>) -> f32 {
    - genome.compare(reference).exp()
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
            //let genome = Genome::create_like(Some(genome_size));
            let genome = create_runner(genome_size);
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

fn create_runner(size: usize) -> Genome<RGB> {
    let root = (size as f32).sqrt() as usize;
    let mut genome = Vec::with_capacity(size);
    for i in 0..size {
        if i < root {
            genome.push(RGB { r: 255, g: 0, b: 0 });
        }
        else {
            genome.push(RGB { r: 0, g: 0, b: 0 });
        }
    }
    genome
}

pub fn create_black(size: usize) -> Genome<RGB> {
    let mut genome = Vec::with_capacity(size);
    for i in 0..size {
        genome.push(RGB { r: 0, g: 0, b: 0 });
    }
    genome
}
