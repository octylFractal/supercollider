extern crate nalgebra as na;

use macroquad::{clear_background, next_frame, Color, GREEN};

use crate::shape::{Shape, ShapeData};
use supercollider::rect::Rect;

mod shape;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 420;

fn main() {
    macroquad::Window::new("GJK Visualizer", the_loop());
}

const CLEAR_COLOR: Color = Color([0xFE, 0xFD, 0xE7, 255]);

async fn the_loop() {
    let my_rect = Shape {
        data: ShapeData::Rect(Rect::new(na::Point2::new(200.0, 200.0), 100.0, 100.0)),
        color: GREEN,
    };

    loop {
        clear_background(CLEAR_COLOR);

        my_rect.draw();

        next_frame().await;
    }
}
