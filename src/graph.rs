pub struct Node<T> {
	value: T,
	x: f32,
	y: f32,
}

pub struct Graph<T> {
	nodes: Vec<Node<T>>,
	adjacencies: Vec<f32>,
}