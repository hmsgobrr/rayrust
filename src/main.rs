use raylib::prelude::*;

const SWIDTH: f32 = 800.0;
const SHEIGHT: f32 = 450.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("Raylib Rust")
        .build();

	let mut ball = Ball{
		speed: 150.0,
		rad: 15.0,
		pos: Vector2{x: SWIDTH/2.0, y: SHEIGHT/2.0},
		vel: Vector2{x: 0.0, y: 0.0}
	};

	ball.init();

    while !rl.window_should_close() {
		let dt = rl.get_frame_time();

		ball.update(dt);
    
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        ball.draw(&mut d);
    }
}

struct Ball {
	speed: f32,
	rad: f32,
	pos: Vector2,
	vel: Vector2
}

impl Ball {
	fn init(&mut self) {
		self.pos.x = SWIDTH/2.0;
		self.pos.y = SHEIGHT/2.0;
		self.vel.x = if get_random_value::<i32>(0, 1) == 0 { -1.0 } else { 1.0 };
		self.vel.y = if get_random_value::<i32>(0, 1) == 0 { -1.0 } else { 1.0 };
	}

	fn update(&mut self, dt: f32) {
		// if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
			// self.vel.y = -1.0;
		// } else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
			// self.vel.y = 1.0;
		// } else {
			// self.vel.y = 0.0;
		// }

		self.pos.x += self.vel.x * self.speed * dt;
		self.pos.y += self.vel.y * self.speed * dt;
	}

	fn draw(&self, d: &mut RaylibDrawHandle) {
		d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.rad, Color::WHITE);
	}
}
