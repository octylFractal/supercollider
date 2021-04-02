use std::ops::Sub;

use na::{DimSub, Point2, Unit, Vector2, U2};

use crate::shape::{support_from_vertices, ConvexShape, SupportProvider};

pub struct Rect {
    center: Point2<f64>,
    pub upper_left: Point2<f64>,
    pub upper_right: Point2<f64>,
    pub lower_left: Point2<f64>,
    pub lower_right: Point2<f64>,
}

impl Rect {
    pub fn new(center: Point2<f64>, width: f64, height: f64) -> Rect {
        let hw = width / 2.0;
        let hh = height / 2.0;
        Rect {
            center: center.clone(),
            upper_left: Point2 {
                coords: center.coords.sub(Vector2::new(-hw, -hh)),
            },
            upper_right: Point2 {
                coords: center.coords.sub(Vector2::new(hw, -hh)),
            },
            lower_left: Point2 {
                coords: center.coords.sub(Vector2::new(-hw, hh)),
            },
            lower_right: Point2 {
                coords: center.coords.sub(Vector2::new(hw, hh)),
            },
        }
    }
}

impl ConvexShape<U2> for Rect {
    fn center(&self) -> Point2<f64> {
        self.center.clone()
    }

    fn support_provider(&self) -> Box<dyn SupportProvider<U2> + '_> {
        Box::from(RectSupportProvider { owner: self })
    }
}

pub struct RectSupportProvider<'a> {
    owner: &'a Rect,
}

impl<'a> SupportProvider<U2> for RectSupportProvider<'a> {
    fn support(&self, dir: Unit<Vector2<f64>>) -> Point2<f64> {
        support_from_vertices::<U2>(
            &[
                self.owner.upper_left.coords.clone(),
                self.owner.upper_right.coords.clone(),
                self.owner.lower_left.coords.clone(),
                self.owner.lower_right.coords.clone(),
            ],
            dir,
        )
    }
}
