extern crate nalgebra as na;

use na::{Point2, U2};

use crate::circle::Circle;
use crate::gjk::GJK;
use crate::rect::Rect;
use crate::shape::ConvexShape;
use std::iter::Once;

mod circle;
mod gjk;
mod rect;
mod shape;

fn main() {
    let mut collision_hash = 1;
    for _ in 0..10_000_000 {
        let shape_a = random_shape();
        let shape_b = random_shape();
        collision_hash += shape_a.iter().check_collision(shape_b.iter()) as i32;
    }
    eprintln!("{:?}", collision_hash);
}

fn random_shape() -> RandomShape {
    if rand::random() {
        RandomShape::Rect(Rect::new(
            Point2::new(rand::random::<f64>() * 100.0, rand::random::<f64>() * 100.0),
            rand::random::<f64>() * 100.0,
            rand::random::<f64>() * 100.0,
        ))
    } else {
        RandomShape::Circle(Circle::new(
            Point2::new(rand::random::<f64>() * 100.0, rand::random::<f64>() * 100.0),
            rand::random::<f64>() * 100.0,
        ))
    }
}
enum RandomShape {
    Rect(Rect),
    Circle(Circle),
}

impl RandomShape {
    fn iter<'a>(&'a self) -> Once<&'a dyn ConvexShape<U2>> {
        match self {
            RandomShape::Rect(ref r) => std::iter::once(r),
            RandomShape::Circle(ref c) => std::iter::once(c),
        }
    }
}
