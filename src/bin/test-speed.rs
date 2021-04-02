extern crate nalgebra as na;

use std::time::Instant;

use na::{Point, Point2, Unit, VectorN, U2};

use supercollider::circle::Circle;
use supercollider::gjk::check_collision;
use supercollider::poly::ConvexPolygon;
use supercollider::shape::{ConvexShape, SupportProvider};

fn main() {
    let random_shapes = (0..10_000_000)
        .into_iter()
        .map(|_| (random_shape(), random_shape()))
        .collect::<Vec<_>>();
    let start = Instant::now();
    let mut collision_hash = 1;
    for (shape_a, shape_b) in random_shapes {
        collision_hash += check_collision(&shape_a, &shape_b) as i32;
    }
    let elapsed = start.elapsed();
    eprintln!("{:?} 10 million in {:?}", collision_hash, elapsed);
}

fn random_shape() -> RandomShape {
    if rand::random() {
        RandomShape::Rect(ConvexPolygon::new_rect(
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
    Rect(ConvexPolygon<4>),
    Circle(Circle),
}

impl ConvexShape<U2, RandomShapeSupportProvider> for RandomShape {
    fn center(&self) -> Point<f64, U2> {
        match self {
            RandomShape::Rect(ref r) => r.center(),
            RandomShape::Circle(ref c) => c.center(),
        }
    }

    fn support_provider<'a>(&'a self) -> RandomShapeSupportProvider {
        match self {
            RandomShape::Rect(ref r) => RandomShapeSupportProvider {
                delegate: Box::from(r.support_provider::<'a>()),
            },
            RandomShape::Circle(ref c) => RandomShapeSupportProvider {
                delegate: Box::from(c.support_provider()),
            },
        }
    }
}

struct RandomShapeSupportProvider {
    delegate: Box<dyn SupportProvider<U2>>,
}

impl<'a> SupportProvider<U2> for RandomShapeSupportProvider {
    fn support(&self, dir: Unit<VectorN<f64, U2>>) -> Point<f64, U2> {
        self.delegate.support(dir)
    }
}
