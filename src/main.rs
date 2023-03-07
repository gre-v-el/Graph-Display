mod controls;
mod helper;
mod graph;

use controls::Controls;
use graph::Graph;
use macroquad::{prelude::*, rand};

#[macroquad::main("graph")]
async fn main() {
	let mut controls = Controls::new();
	let mut graph = Graph::<usize>::new();

	for i in 0..10 {
		graph.add_node(i, rand::gen_range(-15.0, 15.0), rand::gen_range(-15.0, 15.0));
	}
	for _ in 0..10 {
		let i = rand::gen_range(0, graph.num_nodes());
		let j = rand::gen_range(0, graph.num_nodes());

		graph.set_adjacency(i, j, rand::gen_range(1.0, 10.0));
	}

    loop {
		clear_background(BLACK);
		controls.update();
		set_camera(controls.camera());

		draw_grid(&controls);
		graph.draw();


		
		next_frame().await
	}
}

fn draw_grid(controls: &Controls) {
	let pixel = (controls.camera().screen_to_world((0.0, 0.0).into()) - controls.camera().screen_to_world((0.0, 1.0).into())).y;
	let left_top = controls.camera().screen_to_world((0.0, 0.0).into());
	let right_bottom = controls.camera().screen_to_world((screen_width(), screen_height()).into());
	let margin = 1.0f32;
	let area = Rect::new(
		left_top.x - margin, 
		right_bottom.y - margin, 
		right_bottom.x - left_top.x + 2.0*margin, 
		left_top.y - right_bottom.y + 2.0*margin
	);

	let target_lines = 40.0;
	let spacing = (area.w/target_lines).log2().floor().exp2();
	let odd_opacity = ((area.w/target_lines) - spacing)/spacing;
	let odd_opacity = (odd_opacity*3.0).min(1.0);
	let odd_opacity = 1.0 - odd_opacity;
	let opacity = 0.3;

	let start_x = (area.left() / spacing / 2.0).floor()*spacing*2.0;
	let start_y = (area.top() /  spacing / 2.0).floor()*spacing*2.0;
	let steps_x = (area.w / spacing).ceil() as usize;
	let steps_y = (area.h / spacing).ceil() as usize + 1;

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