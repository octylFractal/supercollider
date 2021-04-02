use std::ops::Mul;

use na::{Point2, Unit, Vector2, U2};

use crate::shape::{ConvexShape, SupportProvider};

pub struct Circle {
    center: Point2<f64>,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point2<f64>, radius: f64) -> Circle {
        Circle { center, radius }
    }
}

impl<'a> ConvexShape<U2> for Circle {
    fn center(&self) -> Point2<f64> {
        self.center.clone()
    }

    fn support_provider(&self) -> Box<dyn SupportProvider<U2> + '_> {
        Box::from(CircleSupportProvider { owner: self })
    }
}

pub struct CircleSupportProvider<'a> {
    owner: &'a Circle,
}

impl<'a> SupportProvider<U2> for CircleSupportProvider<'a> {
    fn support(&self, dir: Unit<Vector2<f64>>) -> Point2<f64> {
        Point2::from((dir.mul(self.owner.radius)) + &self.owner.center.coords)
    }
}
