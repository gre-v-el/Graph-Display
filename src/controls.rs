use macroquad::prelude::*;
use crate::helper::*;

pub struct Controls {
	target: Camera2D,
	camera: Camera2D,
	pub mouse_world: Vec2,
	last_mouse_world: Vec2,
	pub drag: Vec2,
}

impl Controls {
	pub fn new() -> Self {
		let camera = Camera2D {
			target: vec2(0.0, 0.0),
			zoom: vec2(0.05, -0.05 * screen_width()/screen_height()),
			..Default::default()
		};

		Controls {
			target: camera.clone(),
			camera,
			mouse_world: vec2(0.0, 0.0),
			last_mouse_world: vec2(0.0, 0.0),
			drag: vec2(0.0, 0.0),
		}
	}

	pub fn update(&mut self) {

		let mouse_screen: Vec2 = mouse_position().into();
		self.mouse_world = self.target.screen_to_world(mouse_screen);
		
		let (_, d_zoom) = 
			if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
				(0.0, 0.0)
			} else {
				mouse_wheel()
			};

		if d_zoom != 0.0 {
			self.target.target = self.mouse_world;

			self.target.zoom.x = self.target.zoom.x * 1.001f32.powf(d_zoom);
			self.target.zoom.y = -self.target.zoom.x * screen_width() / screen_height();

			let mouse_world = self.target.screen_to_world(mouse_screen);

			self.target.target += self.target.target - mouse_world;
		}
		else {
			self.target.zoom.y = -self.target.zoom.x * screen_width() / screen_height();
		}

		if is_mouse_button_down(MouseButton::Right) {
			self.target.target -= self.mouse_world - self.last_mouse_world;
		}
		
		self.camera.target = lerp(self.camera.target, self.target.target, 1.0 - 0.1f32.powf(10.0*get_frame_time()));
		self.camera.zoom =   lerp(self.camera.zoom,   self.target.zoom,   1.0 - 0.1f32.powf(10.0*get_frame_time()));

		self.drag = self.mouse_world - self.last_mouse_world;
        self.last_mouse_world = self.target.screen_to_world(mouse_screen);
	}

	pub fn camera(&self) -> &Camera2D {
		&self.camera
	}
}


pub fn draw_grid(controls: &Controls) {
	let pixel = (controls.camera().screen_to_world((0.0, 0.0).into()) - controls.camera().screen_to_world((0.0, 1.0).into())).y.abs();
	let left_top = controls.camera().screen_to_world((0.0, 0.0).into());
	let right_bottom = controls.camera().screen_to_world((screen_width(), screen_height()).into());
	let margin = 1.0f32;
	let area = Rect::new(
		left_top.x - margin, 
		right_bottom.y + margin, 
		right_bottom.x - left_top.x + 2.0*margin, 
		left_top.y - right_bottom.y - 2.0*margin
	);

	let target_lines = 20.0;
	let spacing = (area.w/target_lines).log2().floor().exp2();
	let odd_opacity = ((area.w/target_lines) - spacing)/spacing;
	let odd_opacity = (odd_opacity*3.0).min(1.0);
	let odd_opacity = 1.0 - odd_opacity;
	let opacity = 0.3;

	let start_x = (area.left() / spacing / 2.0).floor()*spacing*2.0;
	let start_y = (area.bottom() /  spacing / 2.0).floor()*spacing*2.0;
	let steps_x = (area.w / spacing).ceil() as usize;
	let steps_y = (area.h.abs() / spacing).ceil() as usize + 1;

	let width = pixel;

	for i in 0..=steps_x {
		let x = i as f32;
		let col = if i%2 == 0 {Color::new(1.0, 1.0, 1.0, opacity)} else {Color::new(1.0, 1.0, 1.0, odd_opacity*opacity)};
		draw_line(start_x + x*spacing, area.top(), start_x + x*spacing, area.bottom(), width, col);
	}
	for i in 0..=steps_y {
		let y = i as f32;
		let col = if i%2 == 0 {Color::new(1.0, 1.0, 1.0, opacity)} else {Color::new(1.0, 1.0, 1.0, odd_opacity*opacity)};
		draw_line(area.left(), start_y + y*spacing, area.right(), start_y + y*spacing, width, col);
	}
}