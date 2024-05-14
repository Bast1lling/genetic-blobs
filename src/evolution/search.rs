use nannou::math::num_traits::Pow;
use rand::Rng;
use super::blob::Gene;

/// T: state type, U: genome alphabet
pub fn genetic_algorithm<T>(population: &mut Vec<T>) -> Vec<T> 
where T: Gene + Clone{
    let size = population.len();
    let mut new_population = Vec::with_capacity(size);
    weight(population);
    for _ in 0..size {
        let parents = get_parents(population, 5);
        let mut child = reproduce(parents);
        mutate(&mut child, 0.9);
        new_population.push(child);
    }

    new_population
}

fn weight<T>(population: &mut Vec<T>)
where T: Gene {
    population.sort_unstable_by_key(|p| p.rate_fitness() as i32);
}

fn get_parents<T>(population: &Vec<T>, rho: usize) -> Vec<T> 
where T: Clone {
    let mut parents = Vec::with_capacity(rho);
    let p = 1.0/rho as f32;
    while parents.len() < rho {
        let index: usize = rnd_exp(p);
        parents.push(population[index % population.len()].clone());
    }
    parents
}

fn reproduce<T>(mut parents: Vec<T>) -> T 
where T: Gene {
    assert!(parents.len() > 0, "There needs to be at least one parent");
    let mut rng = rand::thread_rng();
    let size = parents[0].length();

    // returns a random stepsize to partition the parents
    let mut get_rand_step = |a: usize, b: usize| {
        let result = (a / b) as i32;
        let mut adjust = rnd_exp(0.3) as i32;
        if 0.5 < rng.gen() {
            adjust *= -1;
        }
        (result + adjust).clamp(0, a.try_into().unwrap()) as usize
    };

    // fit together a child
    let amount_of_cuts = parents.len() - 1;
    let mut cuts = Vec::with_capacity(amount_of_cuts);
    let mut split_index = get_rand_step(size - 1, parents.len());
    while split_index < size && cuts.len() < amount_of_cuts{
        cuts.push(split_index);
        split_index += get_rand_step(size - 1, parents.len());
    }
    cuts.reverse();
    T::combine(parents, cuts)
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

