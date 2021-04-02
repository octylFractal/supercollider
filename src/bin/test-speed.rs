extern crate nalgebra as na;

use na::{Point2, U2};

use supercollider::circle::Circle;
use supercollider::gjk::GJK;
use supercollider::rect::Rect;
use supercollider::shape::ConvexShape;
use std::iter::Once;
use std::time::Instant;

fn main() {
    let random_shapes = (0..10_000_000).into_iter()
        .map(|_| (random_shape(), random_shape()))
        .collect::<Vec<_>>();
    let start = Instant::now();
    let mut collision_hash = 1;
    for (shape_a, shape_b) in random_shapes {
        collision_hash += shape_a.iter().check_collision(shape_b.iter()) as i32;
    }
    let elapsed = start.elapsed();
    eprintln!("{:?} 10 million in {:?}", collision_hash, elapsed);
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
