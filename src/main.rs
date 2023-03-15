mod controls;
mod helper;
mod graph;

use std::time::SystemTime;

use controls::{Controls, draw_grid};
use graph::Graph;
use macroquad::{prelude::*, rand};

#[macroquad::main("graph")]
async fn main() {
	rand::srand(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs());

	let font = Font::default();

	let mut controls = Controls::new();
	
	let mut graph = generate_random_graph();
	// let mut graph = generate_cloth(10);

	let mut selected = 0;

    loop {
		clear_background(BLACK);
		controls.update();
		set_camera(controls.camera());

		draw_grid(&controls);

		graph.spring_update();
		// graph.lerp_update();
		graph.separate_nodes_update(20.0);
		graph.friction_update(0.3);
		graph.kinematic_update(0.2);
		graph.draw(&|v| { format!("{:?}", v) }, font);

		if is_key_down(KeyCode::R) {
			graph = generate_random_graph();
			// graph = generate_cloth(10);
		}

		if is_mouse_button_pressed(MouseButton::Left) {
			selected = graph.nearest_node(controls.camera().screen_to_world(mouse_position().into()));
		}
		if is_mouse_button_down(MouseButton::Left) {
			graph.drag_node(selected, controls.camera().screen_to_world(mouse_position().into()), 2.0);
		}

		next_frame().await
	}
}

fn generate_random_graph() -> Graph<usize> {
	let mut graph = Graph::<usize>::new();

	for i in 0..30 {
		graph.add_node(i, rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0));
	}
	for _ in 0..50 {
		let i = rand::gen_range(0, graph.num_nodes());
		let j = rand::gen_range(0, graph.num_nodes());

		graph.set_adjacency(i, j, rand::gen_range(1.0, 10.0));
	}

	graph
}

fn generate_cloth(side: usize) -> Graph<(usize, usize)> {
	let mut graph = Graph::<(usize, usize)>::new();

	for x in 0..side {
		for y in 0..side {
			graph.add_node((x, y), x as f32, y as f32);

			if x > 0 {
				graph.set_adjacency(x*side + y, (x-1)*side + y, 1.0);
			}
			if y > 0 {
				graph.set_adjacency(x*side + y, x*side + y-1, 1.0);
			}
		}
	}

	graph
}