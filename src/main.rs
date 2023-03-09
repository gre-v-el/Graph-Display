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
		graph.kinematic_update(0.1);
		graph.draw(&|v| { format!("{}", v) }, font);

		if is_key_down(KeyCode::R) {
			graph = generate_random_graph();
		}

		if is_mouse_button_pressed(MouseButton::Left) {
			selected = graph.nearest_node(controls.camera().screen_to_world(mouse_position().into()));
		}
		if is_mouse_button_down(MouseButton::Left) {
			graph.drag_node(selected, controls.camera().screen_to_world(mouse_position().into()));
		}

		next_frame().await
	}
}

fn generate_random_graph() -> Graph<usize> {
	let mut graph = Graph::<usize>::new();

	for i in 0..10 {
		graph.add_node(i, rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0));
	}
	for _ in 0..15 {
		let i = rand::gen_range(0, graph.num_nodes());
		let j = rand::gen_range(0, graph.num_nodes());

		graph.set_adjacency(i, j, rand::gen_range(1.0, 10.0));
	}

	graph
}