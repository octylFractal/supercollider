use std::ops::Sub;

use na::{Point2, Unit, Vector2, U2};

use crate::shape::{support_from_vertices, ConvexShape, SupportProvider};

pub struct ConvexPolygon<const V: usize> {
    center: Point2<f64>,
    pub vertices: [Point2<f64>; V],
}

impl ConvexPolygon<4> {
    pub fn new_rect(center: Point2<f64>, width: f64, height: f64) -> Self {
        let hw = width / 2.0;
        let hh = height / 2.0;
        ConvexPolygon {
            center: center.clone(),
            vertices: [
                Point2 {
                    coords: center.coords.sub(Vector2::new(-hw, -hh)),
                },
                Point2 {
                    coords: center.coords.sub(Vector2::new(hw, -hh)),
                },
                Point2 {
                    coords: center.coords.sub(Vector2::new(hw, hh)),
                },
                Point2 {
                    coords: center.coords.sub(Vector2::new(-hw, hh)),
                },
            ],
        }
    }
}

impl<'a, const V: usize> ConvexShape<U2, ConvexPolygonSupportProvider<'a, V>> for &'a ConvexPolygon<V> {
    fn center(&self) -> Point2<f64> {
        self.center.clone()
    }

    fn support_provider(&self) -> ConvexPolygonSupportProvider<'a, V> {
        ConvexPolygonSupportProvider { owner: self }
    }
}

pub struct ConvexPolygonSupportProvider<'a, const V: usize> {
    owner: &'a ConvexPolygon<V>,
}

impl<'a, const V: usize> SupportProvider<U2> for ConvexPolygonSupportProvider<'a, V> {
    fn support(&self, dir: Unit<Vector2<f64>>) -> Point2<f64> {
        support_from_vertices::<U2>(&self.owner.vertices, dir).clone()
    }
}
