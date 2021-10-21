use raylib::prelude::*;

const SWIDTH: f32 = 800.0;
const SHEIGHT: f32 = 450.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("Raylib Rust")
        .build();

	let mut ball = Ball{ speed: 150.0, rad: 10.0, pos: Vector2{ x: 0.0, y: 0.0 }, vel: Vector2{ x: 0.0, y: 0.0 } };
	ball.init();

	let mut paddle1 = Paddle{ is_left: true, speed: 150.0, rec: Rectangle{ x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, vel_y: 0.0 };
	paddle1.init();
	let mut paddle2 = Paddle{ is_left: false, speed: 150.0, rec: Rectangle{ x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, vel_y: 0.0 };
	paddle2.init();

	let mut score1 = 0;
	let mut score2 = 0;

    while !rl.window_should_close() {
		let dt = rl.get_frame_time();

		ball.update(dt);
		paddle1.update(dt, &mut ball, &rl);
		paddle2.update(dt, &mut ball, &rl);

		if ball.pos.x < 0.0 {
			score1 += 1;
			ball.init();
			paddle1.init();
			paddle2.init();
		}
		if ball.pos.x > SWIDTH {
			score2 += 1;
			ball.init();
			paddle1.init();
			paddle2.init();
		}

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

		d.draw_line((SWIDTH as i32)/2, 0, (SWIDTH as i32)/2, SHEIGHT as i32, Color::WHITE);

		d.draw_text(&score2.to_string(), (SWIDTH as i32)/2 - 30 - measure_text(&score1.to_string(), 50), 10, 50, Color::WHITE);
		d.draw_text(&score1.to_string(), (SWIDTH as i32)/2 + 30, 10, 50, Color::WHITE);

        ball.draw(&mut d);
        paddle1.draw(&mut d);
        paddle2.draw(&mut d);
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
		if self.pos.y + self.rad + 1.0 > SHEIGHT { self.vel.y = -1.0; }
		if self.pos.y - self.rad - 1.0 < 0.0 { self.vel.y = 1.0; }

		self.pos.x += self.vel.x * self.speed * dt;
		self.pos.y += self.vel.y * self.speed * dt;
	}

	fn draw(&self, d: &mut RaylibDrawHandle) {
		d.draw_circle(self.pos.x as i32, self.pos.y as i32, self.rad, Color::WHITE);
	}
}

struct Paddle {
	is_left: bool,
	speed: f32,
	rec: Rectangle,
	vel_y: f32
}

impl Paddle {
	fn init(&mut self) {
		self.rec.x = if self.is_left { 30.0 } else { SWIDTH - 60.0 };
		self.rec.y = SHEIGHT/2.0 - 30.0;
		self.rec.width = 15.0;
		self.rec.height = 60.0;
		self.vel_y = 0.0;
	}

	fn update(&mut self, dt: f32, ball: &mut Ball, rl: &RaylibHandle) {
		if rl.is_key_down(if self.is_left { raylib::consts::KeyboardKey::KEY_W } else { raylib::consts::KeyboardKey::KEY_UP }) {
			self.vel_y = -1.0;
		} else if rl.is_key_down(if self.is_left { raylib::consts::KeyboardKey::KEY_S } else { raylib::consts::KeyboardKey::KEY_DOWN }) {
			self.vel_y = 1.0;
		} else {
			self.vel_y = 0.0;
		}

		self.rec.y += self.vel_y * self.speed * dt;

		if self.rec.check_collision_circle_rec(ball.pos, ball.rad) && self.is_left && ball.pos.x < self.rec.x + 15.0 { ball.vel.x = 1.0; }
		if self.rec.check_collision_circle_rec(ball.pos, ball.rad) && !self.is_left && ball.pos.x > self.rec.x { ball.vel.x = -1.0; }
	}

	fn draw(&self, d: &mut RaylibDrawHandle) {
		d.draw_rectangle_rec(self.rec, Color::WHITE);
	}
}
