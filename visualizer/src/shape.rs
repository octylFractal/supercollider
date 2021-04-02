use na::coordinates::XY;
use na::U2;

use raqote::{DrawOptions, DrawTarget, PathBuilder, Source, StrokeStyle};
use supercollider::circle::Circle;
use supercollider::poly::ConvexPolygon;
use supercollider::shape::ConvexShape;

use num_traits::float::FloatConst;

pub struct ShapeEntity<'a, C: ConvexShape<U2>> {
    pub shape: C,
    pub color: Source<'a>,
}

impl<'a, C: ConvexShape<U2> + DrawConvexShape> ShapeEntity<'a, C> {
    pub fn draw(&self, draw_target: &mut DrawTarget) {
        self.shape.draw(draw_target, &self.color);
    }
}

pub trait DrawConvexShape {
    fn draw(&self, draw_target: &mut DrawTarget, color: &Source);
}

impl<const V: usize> DrawConvexShape for ConvexPolygon<V> {
    fn draw(&self, draw_target: &mut DrawTarget, color: &Source) {
        let mut pb = PathBuilder::new();

        {
            let &XY { x: zx, y: zy } = &*self.vertices[0];
            pb.move_to(zx as f32, zy as f32);
        }

        let count = self.vertices.len();
        for i in 1..count {
            let &XY { x, y } = &*self.vertices[i];
            pb.line_to(x as f32, y as f32);
        }

        pb.close();

        draw_target.stroke(
            &pb.finish(),
            color,
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );
    }
}

impl DrawConvexShape for Circle {
    fn draw(&self, draw_target: &mut DrawTarget, color: &Source) {
        let &XY { x, y } = &*self.center();

        let mut pb = PathBuilder::new();

        pb.arc(x as f32, y as f32, self.radius as f32, 0.0, 2.0 * f32::TAU());

        draw_target.stroke(
            &pb.finish(),
            color,
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );
    }
}
