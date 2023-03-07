use macroquad::prelude::*;

struct Node<T> {
	value: T,
	x: f32,
	y: f32,
}

pub struct Graph<T> {
	nodes: Vec<Node<T>>,
	adjacencies: Vec<f32>,
	max_adjacency: f32,
}

impl<T> Graph<T> {
	pub fn new() -> Self {
		Self {
			nodes: Vec::new(),
			adjacencies: Vec::new(),
			max_adjacency: 0.0,
		}
	}

	pub fn num_nodes(&self) -> usize {
		self.nodes.len()
	}

	pub fn add_node(&mut self, value: T, x: f32, y: f32) {
		self.nodes.push(Node {value, x, y});
		while self.adjacencies.len() < self.nodes.len()*(self.nodes.len()+1)/2 {
			self.adjacencies.push(0.0);
		}
	}

	pub fn set_adjacency(&mut self, i: usize, j: usize, v: f32) {
		if i.max(j) >= self.nodes.len() || i == j { return; }

		let a = i.max(j);
		let b = i.min(j);

		let index = a*(a-1)/2+b;

		self.max_adjacency = self.max_adjacency.max(v);
		self.adjacencies[index] = v;
	}

	pub fn get_adjacency(&self, i: usize, j: usize) -> f32 {
		if i.max(j) >= self.nodes.len() || i == j { return 0.0; }

		let a = i.max(j);
		let b = i.min(j);

		let index = a*(a-1)/2+b;

		self.adjacencies[index]
	}

	pub fn draw(&self) {
		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				let adj = self.get_adjacency(a, b);

				draw_line(self.nodes[a].x, self.nodes[a].y, self.nodes[b].x, self.nodes[b].y, adj/20.0, WHITE);
			}
		}

		for n in &self.nodes {
			draw_circle(n.x, n.y, 1.0, RED);
		}
	}

	pub fn lerp_update(&mut self) {
		if self.max_adjacency == 0.0 { return; }

		let t = 0.1;

		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				let adj = self.get_adjacency(a, b);
				let adj_norm = adj/self.max_adjacency;

				let target_dist = 18.0 - 14.0*adj_norm;
				
				let direction = (self.nodes[b].x - self.nodes[a].x, self.nodes[b].y - self.nodes[a].y);
				let dist = direction.0.hypot(direction.1);
				let direction = (direction.0 / dist, direction.1 / dist);

				let displacement = dist - target_dist;

				self.nodes[a].x += direction.0*displacement*t*(0.5+0.5*adj_norm);
				self.nodes[a].y += direction.1*displacement*t*(0.5+0.5*adj_norm);

				self.nodes[b].x -= direction.0*displacement*t*(0.5+0.5*adj_norm);
				self.nodes[b].y -= direction.1*displacement*t*(0.5+0.5*adj_norm);
			}
		}
	}
}