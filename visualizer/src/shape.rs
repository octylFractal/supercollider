use macroquad::{draw_circle, draw_rectangle, Color};
use supercollider::circle::Circle;
use supercollider::rect::Rect;
use supercollider::shape::ConvexShape;

pub struct Shape {
    pub color: Color,
    pub data: ShapeData,
}

pub enum ShapeData {
    Rect(Rect),
    Circle(Circle),
}

impl Shape {
    pub fn draw(&self) {
        match &self.data {
            ShapeData::Rect(r) => draw_rectangle(
                r.upper_left.x as f32,
                r.upper_left.y as f32,
                (r.lower_right.x - r.upper_left.x) as f32,
                (r.lower_right.y - r.upper_left.y) as f32,
                self.color.clone(),
            ),
            ShapeData::Circle(c) => {
                let &na::coordinates::XY { x, y } = &*c.center();
                draw_circle(x as f32, y as f32, c.radius as f32, self.color.clone());
            }
        }
    }
}
