use nannou::glam::Vec2;

use crate::{
    evolution::{blob::RGB, gene::{CostFunction, Evolve}, population::{
        red_ratio, Population, SimpleBlobPopulation
    }},
    util::Create, 
    Nannou
};

pub trait Control<T>
where
    T: Create + Clone + Copy,
{
}

pub struct SimpleBlobController {
    pub population: SimpleBlobPopulation,
}

impl Nannou for SimpleBlobController {
    fn draw(&self, draw: &nannou::prelude::Draw, model: &crate::Model) {
        for blob in &self.population {
            blob.draw(&draw, &model);
        }
    }

    fn update(&mut self) {
        let genome_references = self.population.extract_genomes();
        SimpleBlobPopulation::evolve(genome_references);

        for blob in self.population.iter_mut() {
            blob.update();
        }
    }
}

impl Create for SimpleBlobController {
    type Params = (Vec<Vec2>, f32, u16, usize);

    fn create() -> Self {
        todo!()
    }

    fn create_like(params: Option<Self::Params>) -> Self {
        let params = params.unwrap();
        let cost_function: CostFunction<RGB> = red_ratio;
        let sbp_params = (params.0, params.1, params.2, params.3, cost_function);
        let population = SimpleBlobPopulation::create_like(Some(sbp_params));

        SimpleBlobController { population }
    }
}
