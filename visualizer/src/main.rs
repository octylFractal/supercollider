extern crate nalgebra as na;

use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use raqote::{DrawTarget, SolidSource, Source};

use supercollider::{
    poly::ConvexPolygon,
    circle::Circle,
    shape::ConvexShape,
    gjk::GJK,
};

use crate::shape::ShapeEntity;
use na::Point2;

mod shape;

fn main() {
    let mut window = Window::new(
        "GJK Visualizer",
        640,
        400,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let clear_color = SolidSource::from_unpremultiplied_argb(0xFF, 0xFE, 0xFD, 0xE7);
    let mut shape_a = ShapeEntity {
        shape: ConvexPolygon::new_rect(na::Point2::new(200.0, 200.0), 100.0, 100.0),
        color: Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xFF, 0x00, 0x00, 0xAA,
        )),
    };
    let mut shape_b = ShapeEntity {
        shape: Circle::new(na::Point2::new(150.0, 150.0), 50.0),
        color: Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xFF, 0x00, 0xAA, 0x00,
        )),
    };

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw
        dt.clear(clear_color);

        shape_a.draw(&mut dt);
        shape_b.draw(&mut dt);

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();

        // Input handling
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        if window.get_mouse_down(MouseButton::Left) {
            let fake_mouse_rect = ConvexPolygon::new_rect(
                Point2::new(x as f64, y as f64),
                1.0, 1.0
            );
            if fake_mouse_rect.iter().check_collision(
                shape_a.shape.iter()
            ) {

            }
        }

        }
    }
}
