use nannou::glam::Vec2;

use crate::{
    evolution::{
        blob::RGB,
        gene::{CostFunction, Creature, Evolve, Genome},
        population::{compare_to, create_black, extract_velocity, move_to, SimpleBlobPopulation}, square::Square,
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
    pub cost_function: CostFunction<Genome<RGB>>,
}

impl Nannou for SimpleBlobController {
    fn draw(&self, draw: &nannou::prelude::Draw, model: &crate::Model) {
        for blob in &self.population {
            blob.draw(&draw, &model);
        }
    }

    fn update(&mut self) {
        SimpleBlobPopulation::evolve(&mut self.population, self.cost_function);

        for blob in self.population.iter_mut() {
            let genome = blob.extract_genome();
            let velocity = extract_velocity(genome);
            blob.velocity = 10.0 * velocity;
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
        // TODO: fix the hardcoded stuff
        let mut reference = create_black(12*12);
        reference.set_quadrant(crate::evolution::square::Quadrant::RightTriangularQuadrant, RGB { r: 255, g: 0, b: 0 });
        let cost_function: CostFunction<Genome<RGB>> = compare_to_red;
        let population = SimpleBlobPopulation::create_like(params);
        SimpleBlobController {
            population,
            cost_function,
        }
    }
}

fn compare_to_red(g: &Genome<RGB>) -> f32 {
    let mut reference = create_black(12*12);
    reference.set_quadrant(crate::evolution::square::Quadrant::RightTriangularQuadrant, RGB { r: 255, g: 0, b: 0 });
    compare_to(g, &reference)
}
