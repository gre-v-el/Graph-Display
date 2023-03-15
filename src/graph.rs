use macroquad::prelude::*;

struct Node<T> {
	value: T,
	mass: f32,
	pos: Vec2,
	v: Vec2,
	f: Vec2,
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
		self.nodes.push(Node {value, mass: 1.0, pos: vec2(x, y), v: vec2(0.0, 0.0), f: vec2(0.0, 0.0)});
		while self.adjacencies.len() < self.nodes.len()*(self.nodes.len()+1)/2 {
			self.adjacencies.push(0.0);
		}
	}

	pub fn nearest_node(&self, pos: Vec2) -> usize {
		let mut nearest = 0;
		let mut nearest_dist = f32::MAX;
		for i in 0..self.num_nodes() {
			let dist = pos.distance(self.nodes[i].pos);
			if dist < nearest_dist {
				nearest_dist = dist;
				nearest = i;
			}
		}

		nearest
	}

	pub fn drag_node(&mut self, node: usize, pos: Vec2, strength: f32) {
		// let node_pos = self.nodes[node].pos;
		// self.nodes[node].f += (pos - node_pos)*strength;
		self.nodes[node].pos = pos;
		self.nodes[node].v = vec2(0.0, 0.0);
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

	pub fn draw(&self, to_str: &dyn Fn(&T) -> String, font: Font) {
		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				let adj = self.get_adjacency(a, b);

				draw_line(self.nodes[a].pos.x, self.nodes[a].pos.y, self.nodes[b].pos.x, self.nodes[b].pos.y, adj/20.0, WHITE);
			}
		}

		for n in &self.nodes {
			draw_circle(n.pos.x, n.pos.y, 1.0, RED);
			let txt = to_str(&n.value);

			let dims = measure_text(txt.as_str(), Some(font), 64, 0.02);
			
			draw_text_ex(txt.as_str(), n.pos.x - dims.width/2.0, n.pos.y + dims.height/2.0, TextParams { font: font, font_size: 64, font_scale: 0.02, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });
		}
	}

	pub fn lerp_update(&mut self) {
		if self.max_adjacency == 0.0 { return; }

		for a in 0..self.nodes.len() {
			self.nodes[a].v = vec2(0.0, 0.0);
		}

		let t = 0.3;

		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				let adj = self.get_adjacency(a, b);
				let adj_norm = adj/self.max_adjacency;

				let target_dist = 18.0 - 14.0*adj_norm;
				
				let direction = (self.nodes[b].pos.x - self.nodes[a].pos.x, self.nodes[b].pos.y - self.nodes[a].pos.y);
				let dist = direction.0.hypot(direction.1);
				let direction = (direction.0 / dist, direction.1 / dist);

				let displacement = dist - target_dist;

				self.nodes[a].v.x += direction.0*displacement*t*(0.5+0.5*adj_norm);
				self.nodes[a].v.y += direction.1*displacement*t*(0.5+0.5*adj_norm);

				self.nodes[b].v.x -= direction.0*displacement*t*(0.5+0.5*adj_norm);
				self.nodes[b].v.y -= direction.1*displacement*t*(0.5+0.5*adj_norm);
			}
		}
	}

	pub fn kinematic_update(&mut self, dt: f32) {

		for n in &mut self.nodes {
			n.v += n.f * dt / n.mass;
			n.pos += n.v * dt;
			n.f = vec2(0.0, 0.0);
		}
	}

	pub fn friction_update(&mut self, friction: f32) {
		for n in &mut self.nodes {
			n.f -= n.v * friction;
		}
	}

	pub fn spring_update(&mut self) {
		if self.max_adjacency == 0.0 { return; }

		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				let adj = self.get_adjacency(a, b);
				let adj_norm = adj/self.max_adjacency;

				let target_dist = 20.0 - 20.0*adj_norm;

				let direction = (self.nodes[b].pos.x - self.nodes[a].pos.x, self.nodes[b].pos.y - self.nodes[a].pos.y);
				let dist = direction.0.hypot(direction.1);
				let direction = (direction.0 / dist, direction.1 / dist);

				let displacement = dist - target_dist;

				// F = -kx
				// k - spring constant, set to normalized adjacency

				let k = adj_norm + 0.005;
				self.nodes[a].f.x += k * displacement * direction.0;
				self.nodes[a].f.y += k * displacement * direction.1;

				self.nodes[b].f.x -= k * displacement * direction.0;
				self.nodes[b].f.y -= k * displacement * direction.1;
			}
		}

		
	}

	pub fn separate_nodes_update(&mut self, force: f32) {
		
		for b in 0..self.nodes.len() {
			for a in (b+1)..self.nodes.len() {
				
				let direction = (self.nodes[b].pos.x - self.nodes[a].pos.x, self.nodes[b].pos.y - self.nodes[a].pos.y);
				let dist = direction.0.hypot(direction.1);
				let direction = (direction.0 / dist, direction.1 / dist);
				
				let padding = 1.0;
				let amount = 1.0/(dist + padding)/(dist + padding);
				
				self.nodes[a].f.x -= force * amount * direction.0;
				self.nodes[a].f.y -= force * amount * direction.1;
			
				self.nodes[b].f.x += force * amount * direction.0;
				self.nodes[b].f.y += force * amount * direction.1;
			}
		}
	}

	pub fn straighten_connections_update(&mut self) {
		/*
			b			    	  b
			|						\
			|	         	VS	      a
			|							\
			a-------c			          c
			
			The right one is more readable
			So in the left scenario, apply force to A in the direction of the smaller angle:
			(Conservation of energy shall not be violated! - b and c each get a force of half the value and opposite direction)

			b  F
			| /
			a---c

			F ~ adj(a, b) (for any 0 adjacency do nothing)
			F ~ adj(a, c)
			F ~ 0 when ABC are colinear with A in the middle, 1.0 when colinear with A on an extreme, interpolated

			angle Fab = angle Fac
		 */


		// for each triangle in the graph
		for a in 0..self.nodes.len() {
			for b in 0..self.nodes.len() {
				if a == b { continue; }

				for c in 0..self.nodes.len() {
					if a == c { continue; }

					let dot = (self.nodes[b].pos - self.nodes[a].pos).normalize_or_zero().dot((self.nodes[c].pos - self.nodes[a].pos).normalize_or_zero());
					let dot = dot*0.5 + 0.5;

					let force = 0.01 * self.get_adjacency(a, b) * self.get_adjacency(a, c) * dot;

					let direction = (0.5*(self.nodes[b].pos + self.nodes[c].pos) - self.nodes[a].pos).normalize_or_zero();

					self.nodes[a].f += direction * force;
					self.nodes[b].f -= direction * force * 0.5;
					self.nodes[c].f -= direction * force * 0.5;
				}
			}
		}
	}
}