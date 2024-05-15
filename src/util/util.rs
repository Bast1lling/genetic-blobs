use nannou::geom::Point2;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::f32::consts::PI;

/// distribute *amount* objects uniformly in space
pub fn distribute_uniformly(amount: u32, object_size: f32) -> Vec<Point2> {
    let mut result: Vec<Point2> = Vec::new();
    let sparsity: f32 = 1.0;
    let mut rng = rand::thread_rng();
    distribute_uniformly_recursive(&mut rng, amount, amount, Point2::new(0.,0.), 0., sparsity *object_size, sparsity *object_size, &mut result);
    result
}

fn distribute_uniformly_recursive(rng: &mut ThreadRng, amount: u32,  target: u32, center: Point2, inner_bound: f32, outer_bound: f32, margin_radius: f32, points: &mut Vec<Point2>) {
    if target <= 0 {
        return;
    }
    let randomness = 0.1;
    // free area
    let area_all = calc_area(outer_bound) - calc_area(inner_bound);
    // area of a single object + padding
    let area_single = calc_area(margin_radius);
    // amount of objects to fit in the current space
    let capacity: usize = (area_all / area_single) as usize;

    // random offset from unit circle
    let random_angle = rng.gen_range(0.0..2.0 * PI);
    // step size on unit circle
    let step_size = (2.0 * PI) / capacity as f32;
    // center of free area
    let initial_radius = inner_bound + margin_radius / 2.0;

    let mut index = 0;
    while capacity > index && points.len() < amount as usize {
        let random_angle_offset = (randomness / 2.0) + rng.gen_range(0.0..=randomness);
        let random_radius_offset_x = (randomness / 2.0) + rng.gen_range(0.0..=randomness/2.);
        let random_radius_offset_y = (randomness / 2.0) + rng.gen_range(0.0..=randomness/2.);
        let angle = random_angle + step_size * index as f32 + step_size * random_angle_offset;
        let x = angle.cos();
        let y = angle.sin();
        let new_point = Point2::new(center.x + x * initial_radius + margin_radius * random_radius_offset_x, center.y + y * initial_radius + margin_radius * random_radius_offset_y);
        points.push(new_point);

        index += 1;
    }

    distribute_uniformly_recursive(rng, amount,target - index as u32, center, outer_bound, outer_bound + margin_radius, margin_radius, points);
}

fn calc_area(radius: f32) -> f32 {
    PI * radius * radius
}

/// returns a number x of natural numbers with probability p^x
pub fn rnd_exp(expected: usize) -> usize {
    assert!(expected > 0);
    let mut rng = rand::thread_rng();
    // E = (1/1-p)
    // E * (1-p) = 1
    // 1 - p = 1/E
    // p = -(1/E - 1)
    let p = 1.0 - 1.0 /(expected as f32);
    let mut result: usize = 0;
    
    while rng.gen::<f32>() > p {
        result += 1;
    }

    result
}