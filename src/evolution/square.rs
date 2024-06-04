use std::usize;

use crate::util::Create;
use super::gene::Genome;

/// A Quadrant defines a specific subset of the Square
pub enum Quadrant {
    TopTriangularQuadrant,
    BottomTriangularQuadrant,
    RightTriangularQuadrant,
    LeftTriangularQuadrant,
}

pub trait Square<T: Copy> {
    fn side_length(&self) -> usize;
    fn get(&self, at: (usize, usize)) -> &T;
    fn set(&mut self, at: (usize, usize), value: T);

    fn get_quadrant(&self, quadrant: Quadrant) -> Vec<&T> {
        let indices = Self::get_quadrant_indices(quadrant, self.side_length());
        let mut result = Vec::with_capacity(indices.len());
        for index in indices.iter() {
            result.push(self.get(*index));
        }
        result
    }
    
    fn set_quadrant(&mut self, quadrant: Quadrant, value: T) {
        let indices = Self::get_quadrant_indices(quadrant, self.side_length());
        for index in indices.iter() {
            self.set(*index, value);
        }
    }
    
    fn get_quadrant_indices(quadrant: Quadrant, side_length: usize) -> Vec<(usize, usize)> {
        let half = side_length / 2;
        let mut triangle = Vec::with_capacity(half);
        for i in 0..half {
            let mut row = Vec::with_capacity(side_length - 2*i);
            for j in (i)..(side_length - i) {
               row.push(j);
            }
            triangle.push(row)
        }

        match quadrant {
            Quadrant::TopTriangularQuadrant => {
                let mut result = Vec::with_capacity(half * half + 2*half);
                for (i, row) in triangle.iter().enumerate() {
                    for j in row.iter() {
                        result.push((*j, side_length - 1 - i));
                    }
                }
                result
            },
            Quadrant::BottomTriangularQuadrant => {
                let mut result = Vec::with_capacity(half * half + 2*half);
                for (i, row) in triangle.iter().enumerate() {
                    for j in row.iter() {
                        result.push((*j, i));
                    }
                }
                result
            },
            Quadrant::RightTriangularQuadrant => {
                let mut result = Vec::with_capacity(half * half + 2*half);
                for (i, row) in triangle.iter().enumerate() {
                    for j in row.iter() {
                        result.push((side_length - 1 - i, *j));
                    }
                }
                result
            },
            Quadrant::LeftTriangularQuadrant => {
                let mut result = Vec::with_capacity(half * half + 2*half);
                for (i, row) in triangle.iter().enumerate() {
                    for j in row.iter() {
                        result.push((i, *j));
                    }
                }
                result
            },
        }
    }
}

impl<T: Create + Clone + Copy> Square<T> for Genome<T> {

    fn side_length(&self) -> usize {
        (self.len() as f32).sqrt() as usize
    }

    fn get(&self, at: (usize, usize)) -> &T {
        let (x, y) = at;
        let width = self.side_length();
        &self[y*width + x]
    }
    
    fn set(&mut self, at: (usize, usize), value: T) {
        let (x, y) = at;
        let width = self.side_length();
        self[y*width + x] = value;
    }
}