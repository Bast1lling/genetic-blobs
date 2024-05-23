use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::RGB,
        gene::{CostFunction, Evolve, Population},
        population::{red_ratio, SimpleBlobPopulation},
    },
    util::Create,
    Nannou,
};

pub trait Control<T>
where
    T: Create + Clone + Copy,
{
}

pub struct SimpleBlobController {
    pub population: SimpleBlobPopulation,
    pub cost_function: CostFunction<RGB>,
}

impl Nannou for SimpleBlobController {
    fn draw(&self, draw: &nannou::prelude::Draw, model: &crate::Model) {
        for blob in &self.population {
            blob.draw(&draw, &model);
        }
    }

    fn update(&mut self) {
        let genome_references = self.population.extract_genomes();
        SimpleBlobPopulation::evolve(genome_references, self.cost_function);

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
        let cost_function: CostFunction<RGB> = red_ratio;
        let population = SimpleBlobPopulation::create_like(params);

        SimpleBlobController { population, cost_function }
    }
}
