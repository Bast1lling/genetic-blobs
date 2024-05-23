use crate::{
    evolution::{gene::Evolve, population::SimpleBlobPopulation},
    util::Create,
};

pub struct Statistic {
    fitness_data: Vec<f32>,
}

impl Statistic {
    pub fn read_all(&self) -> Vec<&Vec<f32>> {
        let mut data = Vec::new();
        data.push(&self.fitness_data);

        data
    }

    pub fn update(&mut self, population: &mut SimpleBlobPopulation) {
        todo!()
    }
}
