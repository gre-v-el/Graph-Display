use std::ops::{Mul, Add};

use macroquad::prelude::Color;

pub fn lerp<T>(a: T, b: T, t: f32) -> T 
where 
	f32 : Mul<T, Output = T>,
	T : Add<Output = T>,
{
	t * b + (1.0 - t) * a
}

pub fn col_from_array(ar: [f32; 3]) -> Color {
	[ar[0], ar[1], ar[2], 1.0].into()
}