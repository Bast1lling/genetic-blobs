use nannou::math::num_traits::Pow;
use rand::Rng;
use super::blob::Gene;

/// use the Ts of one generation to generate another
pub fn evolve<T>(population: &mut Vec<T>) -> Vec<T> 
where T: Gene + Clone{
    let size = population.len();
    let mut children = Vec::with_capacity(size);

    weight(population);

    for mother in population.iter() {
        let fathers = get_fathers(population, size/4);
        let mut child = reproduce(mother, &fathers);
        mutate(&mut child, 0.9);
        children.push(child);
    }

    children
}

fn weight<T>(population: &mut Vec<T>)
where T: Gene {
    population.sort_unstable_by_key(|p| p.rate_fitness() as i32);
}

fn get_fathers<T>(population: &Vec<T>, rho: usize) -> Vec<T> 
where T: Clone {
    let mut fathers = Vec::with_capacity(rho);
    let p = 1.0/rho as f32;
    while fathers.len() < rho {
        let index: usize = rnd_exp(p);
        fathers.push(population[index % population.len()].clone());
    }
    fathers
}

/// fathers ordered descending according to their fitness
fn reproduce<T>(mother: &T, fathers: &Vec<T>) -> T 
where T: Gene {
    assert!(fathers.len() < 256);

    // mapper function which maps a genome index to a father
    let map = |_: usize| {
        (rnd_exp(0.2) % (fathers.len() + 1)) as u8
    };

    // figure out the intervals at which genetic information will be copied
    let mut indices = Vec::with_capacity(mother.length());
    for i in 0..mother.length() {
        indices.push(map(i));
    }
    T::combine(mother, fathers, &indices)
}


/// the greater gamma, the more likely are multiple mutations
fn mutate<T>(t: &mut T, gamma: f32)
where T: Gene {
    let mut rng = rand::thread_rng();
    let p: f32 = 1.0 / (1.0 + gamma.pow(t.length() as f32));
    let mutation_amount = rnd_exp(p);
    for _ in 0..mutation_amount {
        let at = rng.gen_range(0..t.length());
        t.mutate(at);
    }
}

/// returns a number x of natural numbers with probability p^x
fn rnd_exp(p: f32) -> usize {
    assert!(0.0 < p && p <= 1.0, "Value is not in the range [0, 1]");
    let mut rng = rand::thread_rng();
    let mut result: usize = 0;
    
    while rng.gen::<f32>() > p {
        result += 1;
    }

    result
}

