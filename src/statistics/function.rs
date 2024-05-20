use crate::evolution::{blob::Gene, search::Evolve};

pub struct Statistic {
    fitness_data: Vec<f32>,
}

impl Statistic {
    pub fn read_all(&self) -> Vec<&Vec<f32>> {
        let mut data = Vec::new();
        data.push(&self.fitness_data);

        data
    }
    
    pub fn update<T, S>(&mut self, population: &mut T)
    where T: Evolve<S>, S: Gene + Clone {
        let best = &population.get()[0];
        self.fitness_data.push(best.rate_fitness());
    }
}