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
			zoom: vec2(0.05, 0.05 * screen_width()/screen_height()),
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
			self.target.zoom.y = self.target.zoom.x * screen_width() / screen_height();

			let mouse_world = self.target.screen_to_world(mouse_screen);

			self.target.target += self.target.target - mouse_world;
		}
		else {
			self.target.zoom.y = self.target.zoom.x * screen_width() / screen_height();
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