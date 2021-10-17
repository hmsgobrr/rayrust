use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Raylib Rust")
        .build();

	let ball = Ball{
		rad: 30.0,
		pos: Vector2{x: 400.0, y:225.0},
		vel: Vector2{x: 0.0, y: 0.0}
	};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_circle(ball.pos.x as i32, ball.pos.y as i32, ball.rad, Color::WHITE);
    }
}

struct Ball {
	rad: f32,
	pos: Vector2,
	vel: Vector2
}
