use std::ops::Sub;

use na::{Point2, Unit, Vector2, U2};

use crate::shape::{support_from_vertices, ConvexShape, SupportProvider};

pub struct Rect {
    center: Point2<f64>,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(center: Point2<f64>, width: f64, height: f64) -> Rect {
        Rect {
            center,
            width,
            height,
        }
    }
}

impl ConvexShape<U2> for Rect {
    fn center(&self) -> Point2<f64> {
        self.center.clone()
    }

    fn support_provider(&self) -> Box<dyn SupportProvider<U2>> {
        let hw = self.width / 2.0;
        let hh = self.height / 2.0;
        Box::from(RectSupportProvider {
            vertices: [
                self.center.coords.sub(Vector2::new(-hw, -hh)),
                self.center.coords.sub(Vector2::new(hw, -hh)),
                self.center.coords.sub(Vector2::new(-hw, hh)),
                self.center.coords.sub(Vector2::new(hw, hh)),
            ],
        })
    }
}

pub struct RectSupportProvider {
    vertices: [Vector2<f64>; 4],
}

impl SupportProvider<U2> for RectSupportProvider {
    fn support(&self, dir: Unit<Vector2<f64>>) -> Point2<f64> {
        support_from_vertices::<U2>(&self.vertices, dir)
    }
}
